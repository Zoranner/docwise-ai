//! 规划 / 执行智能体共用的流式工具循环（`complete_stream` + 多轮 tool）。
//!
//! **重试范围**：仅针对**当前 [`PlanningStreamEvent::ModelRound`] 对应的那一次** `complete_stream`
//!（同一快照 `messages` + tools）。不会从用户回合开头重跑，也不会撤销已执行的工具轮次与已写入的 `messages`。

use std::collections::HashMap;
use std::sync::Arc;

use agentool::Tool;
use futures::StreamExt;
use lmkit::chat::{
    merge_tool_call_deltas, ChatEvent, ChatMessage, ChatProvider, ChatRequest, FinishReason,
    RequestPreset, Role, ToolCall, ToolChoice,
};

use super::llm_retry;
use super::planning::PlanningLoopConfig;
use super::planning_core::{dispatch_tool, parse_tool_arguments};
use super::planning_events::{
    chat_to_wire, finish_reason_str, last_assistant_content, PlanningStreamEnvelope,
    PlanningStreamEvent, ToolCallWire,
};
use super::tool_registry::tools_by_name;
use crate::app::project::tools::definitions_for_lmkit;

#[derive(Debug)]
pub enum AgentStreamError {
    Lmkit(lmkit::Error),
    MaxToolRounds { limit: u32 },
    StreamEndedWithoutFinish,
    EmptyToolCallsAfterToolFinish,
}

impl std::fmt::Display for AgentStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lmkit(e) => write!(f, "{}", e),
            Self::MaxToolRounds { limit } => {
                write!(f, "agent stream exceeded max tool rounds ({limit})")
            }
            Self::StreamEndedWithoutFinish => write!(f, "stream ended without finish event"),
            Self::EmptyToolCallsAfterToolFinish => write!(
                f,
                "model finish_reason tool_calls but no tool calls could be merged"
            ),
        }
    }
}

impl std::error::Error for AgentStreamError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Lmkit(e) => Some(e),
            _ => None,
        }
    }
}

impl From<lmkit::Error> for AgentStreamError {
    fn from(e: lmkit::Error) -> Self {
        Self::Lmkit(e)
    }
}

fn tool_result_ok(out: &serde_json::Value) -> bool {
    out.get("error").is_none()
}

/// 已展开的工具列表 +温度预设 + 角色标签（`planning` / `execution`）。
pub struct AgentStreamTurn<'a> {
    pub provider: &'a dyn ChatProvider,
    pub messages: Vec<ChatMessage>,
    pub tools: Vec<Arc<dyn Tool>>,
    pub config: PlanningLoopConfig,
    pub run_id: &'a str,
    pub provider_label: &'a str,
    pub model_label: &'a str,
    pub preset: RequestPreset,
    pub agent_label: &'a str,
}

pub async fn run_agent_turn_stream<E>(
    turn: AgentStreamTurn<'_>,
    mut emit: E,
) -> Result<Vec<ChatMessage>, AgentStreamError>
where
    E: FnMut(PlanningStreamEnvelope),
{
    let AgentStreamTurn {
        provider,
        mut messages,
        tools,
        config,
        run_id,
        provider_label,
        model_label,
        preset,
        agent_label,
    } = turn;

    let tool_defs = definitions_for_lmkit(&tools);
    let registry: HashMap<String, Arc<dyn Tool>> = tools_by_name(&tools);

    let mut seq: u64 = 0;
    let mut emit_ev = |event: PlanningStreamEvent| {
        seq += 1;
        emit(PlanningStreamEnvelope {
            run_id: run_id.to_string(),
            seq,
            event,
        });
    };

    emit_ev(PlanningStreamEvent::TurnStarted {
        agent: agent_label.to_string(),
        provider: provider_label.to_string(),
        model: model_label.to_string(),
        max_tool_rounds: config.max_tool_rounds,
    });

    let mut tool_rounds_completed: u32 = 0;
    let mut model_round: u32 = 0;

    'agent: loop {
        model_round += 1;
        emit_ev(PlanningStreamEvent::ModelRound {
            round_index: model_round,
        });

        let req = ChatRequest {
            messages: messages.clone(),
            tools: Some(tool_defs.clone()),
            tool_choice: Some(ToolChoice::Auto),
            preset: Some(preset),
            ..Default::default()
        };

        let max_attempts = config.llm_max_retries.saturating_add(1).max(1);
        let mut last_reason = String::new();
        let mut round_ok: Option<(FinishReason, String, Vec<lmkit::chat::ToolCallDelta>)> = None;

        for attempt in 0..max_attempts {
            if attempt > 0 {
                let delay = llm_retry::backoff_delay_ms(
                    attempt - 1,
                    config.llm_retry_base_delay_ms,
                    config.llm_retry_max_delay_ms,
                );
                emit_ev(PlanningStreamEvent::LlmRequestRetry {
                    model_round_index: model_round,
                    attempt: attempt + 1,
                    max_attempts,
                    delay_ms: delay,
                    reason: last_reason.clone(),
                });
                llm_retry::sleep_delay_ms(delay).await;
            }

            let stream = match provider.complete_stream(&req).await {
                Ok(s) => s,
                Err(e) => {
                    if e.is_retryable() && attempt + 1 < max_attempts {
                        last_reason = e.to_string();
                        continue;
                    }
                    emit_ev(PlanningStreamEvent::Error {
                        code: "lmkit".into(),
                        message: e.to_string(),
                        partial_messages: Some(messages.iter().map(chat_to_wire).collect()),
                    });
                    return Err(AgentStreamError::from(e));
                }
            };
            let mut stream = Box::pin(stream);

            let mut text_buf = String::new();
            let mut tool_deltas: Vec<lmkit::chat::ToolCallDelta> = Vec::new();
            let mut last_finish: Option<FinishReason> = None;
            let mut abandon_stream_attempt = false;

            while let Some(item) = stream.next().await {
                let ev = match item {
                    Ok(x) => x,
                    Err(e) => {
                        if e.is_retryable() && attempt + 1 < max_attempts {
                            last_reason = e.to_string();
                            abandon_stream_attempt = true;
                            break;
                        }
                        emit_ev(PlanningStreamEvent::Error {
                            code: "lmkit".into(),
                            message: e.to_string(),
                            partial_messages: Some(messages.iter().map(chat_to_wire).collect()),
                        });
                        return Err(AgentStreamError::from(e));
                    }
                };
                match ev {
                    ChatEvent::Delta(s) => {
                        if !s.is_empty() {
                            emit_ev(PlanningStreamEvent::LlmDelta { text: s.clone() });
                            text_buf.push_str(&s);
                        }
                    }
                    ChatEvent::ToolCallDelta(d) => {
                        tool_deltas.extend(d);
                    }
                    ChatEvent::Finish(r) => {
                        last_finish = Some(r);
                        let content_snap = if text_buf.is_empty() {
                            None
                        } else {
                            Some(text_buf.clone())
                        };
                        emit_ev(PlanningStreamEvent::LlmRoundFinished {
                            finish_reason: finish_reason_str(r).to_string(),
                            content: content_snap,
                        });
                        break;
                    }
                }
            }

            if abandon_stream_attempt {
                continue;
            }

            let fr = match last_finish {
                Some(f) => f,
                None => {
                    if attempt + 1 < max_attempts {
                        last_reason =
                            "SSE/response stream ended without a finish event".to_string();
                        continue;
                    }
                    emit_ev(PlanningStreamEvent::Error {
                        code: "stream_truncated".into(),
                        message: "SSE/response stream ended without a finish event".into(),
                        partial_messages: Some(messages.iter().map(chat_to_wire).collect()),
                    });
                    return Err(AgentStreamError::StreamEndedWithoutFinish);
                }
            };

            round_ok = Some((fr, text_buf, tool_deltas));
            break;
        }

        let (fr, text_buf, tool_deltas) =
            round_ok.expect("internal: LLM retry loop must return or set round_ok before exit");

        let merged: Vec<ToolCall> = if fr == FinishReason::ToolCalls {
            merge_tool_call_deltas(&tool_deltas)
        } else {
            vec![]
        };

        if fr == FinishReason::ToolCalls && merged.is_empty() {
            emit_ev(PlanningStreamEvent::Error {
                code: "empty_tool_calls".into(),
                message: "finish_reason was tool_calls but merged tool call list is empty".into(),
                partial_messages: Some(messages.iter().map(chat_to_wire).collect()),
            });
            return Err(AgentStreamError::EmptyToolCallsAfterToolFinish);
        }

        let assistant_msg = ChatMessage {
            role: Role::Assistant,
            content: if text_buf.is_empty() {
                None
            } else {
                Some(text_buf)
            },
            tool_calls: if merged.is_empty() {
                None
            } else {
                Some(merged.clone())
            },
            tool_call_id: None,
            name: None,
        };
        messages.push(assistant_msg);

        if merged.is_empty() {
            emit_ev(PlanningStreamEvent::TurnFinished {
                last_assistant_text: last_assistant_content(&messages),
                tool_rounds_completed,
                messages: messages.iter().map(chat_to_wire).collect(),
            });
            return Ok(messages);
        }

        tool_rounds_completed += 1;
        if tool_rounds_completed > config.max_tool_rounds {
            emit_ev(PlanningStreamEvent::Error {
                code: "max_tool_rounds".into(),
                message: format!("exceeded limit {}", config.max_tool_rounds),
                partial_messages: Some(messages.iter().map(chat_to_wire).collect()),
            });
            return Err(AgentStreamError::MaxToolRounds {
                limit: config.max_tool_rounds,
            });
        }

        emit_ev(PlanningStreamEvent::ToolCallsParsed {
            calls: merged.iter().map(ToolCallWire::from).collect(),
        });

        for tc in merged {
            emit_ev(PlanningStreamEvent::ToolExecuting {
                tool_call_id: tc.id.clone(),
                name: tc.function.name.clone(),
            });
            let args = parse_tool_arguments(&tc.function.arguments);
            let out = dispatch_tool(&registry, &tc.function.name, args).await;
            let ok = tool_result_ok(&out);
            emit_ev(PlanningStreamEvent::ToolFinished {
                tool_call_id: tc.id.clone(),
                name: tc.function.name.clone(),
                ok,
                result: out.clone(),
            });
            let payload = serde_json::to_string(&out).unwrap_or_else(|_| "{}".into());
            messages.push(ChatMessage::tool_with_name(
                &tc.id,
                &tc.function.name,
                payload,
            ));
        }

        continue 'agent;
    }
}
