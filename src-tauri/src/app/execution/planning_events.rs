//! 规划智能体流式事件与前后端消息线格式（可 JSON 序列化）。

use lmkit::chat::{ChatMessage, FinishReason, FunctionCallResult, Role, ToolCall};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 前端监听固定事件名时使用；与载荷内的 `run_id` 一起过滤会话。
pub const PLANNING_AGENT_EVENT: &str = "docwise:planning-agent";

/// 文档执行智能体流式事件（载荷形状与规划侧相同，见 [`PlanningStreamEnvelope`]）。
pub const EXECUTION_AGENT_EVENT: &str = "docwise:execution-agent";

fn default_planning_agent() -> String {
    "planning".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanningStreamEnvelope {
    pub run_id: String,
    /// 单调递增序号，便于前端在无时钟保证时排序。
    pub seq: u64,
    pub event: PlanningStreamEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PlanningStreamEvent {
    /// 回合开始（含 LLM 配置摘要，便于排错）。
    TurnStarted {
        /// `planning` | `execution`；旧载荷缺省反序列化为 `planning`。
        #[serde(default = "default_planning_agent")]
        agent: String,
        provider: String,
        model: String,
        max_tool_rounds: u32,
    },
    /// 新一轮请求模型前（每轮工具执行之后或首轮）。
    ModelRound {
        round_index: u32,
    },
    /// 同一 **model_round** 内、对单次 `complete_stream` 的 HTTP/SSE 因可重试错误即将退避重试（`attempt` 从 2 起）。
    LlmRequestRetry {
        /// 与 [`ModelRound::round_index`] 一致，标明重试的是哪一次模型请求（非整轮用户回合从头）。
        model_round_index: u32,
        attempt: u32,
        max_attempts: u32,
        delay_ms: u64,
        reason: String,
    },
    /// 流式文本增量（assistant）。
    LlmDelta {
        text: String,
    },
    /// 本段流式结束：携带当前累积的 assistant 文本快照与结束原因。
    LlmRoundFinished {
        finish_reason: String,
        content: Option<String>,
    },
    /// 模型声明的工具调用（已合并 delta）。
    ToolCallsParsed {
        calls: Vec<ToolCallWire>,
    },
    ToolExecuting {
        tool_call_id: String,
        name: String,
    },
    /// `result` 为工具 JSON 结果或错误对象。
    ToolFinished {
        tool_call_id: String,
        name: String,
        ok: bool,
        result: Value,
    },
    /// 整轮用户输入结束：完整轨迹（可持久化）。
    TurnFinished {
        last_assistant_text: Option<String>,
        tool_rounds_completed: u32,
        messages: Vec<ChatMessageWire>,
    },
    Error {
        code: String,
        message: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        partial_messages: Option<Vec<ChatMessageWire>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageWire {
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallWire>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallWire {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

pub fn finish_reason_str(r: FinishReason) -> &'static str {
    match r {
        FinishReason::Stop => "stop",
        FinishReason::Length => "length",
        FinishReason::ContentFilter => "content_filter",
        FinishReason::ToolCalls => "tool_calls",
    }
}

impl From<&ToolCall> for ToolCallWire {
    fn from(tc: &ToolCall) -> Self {
        Self {
            id: tc.id.clone(),
            name: tc.function.name.clone(),
            arguments: tc.function.arguments.clone(),
        }
    }
}

/// 从完整轨迹中取「最后一条 assistant」的正文（用于摘要；最后一条可能是 `tool`）。
pub fn last_assistant_content(msgs: &[ChatMessage]) -> Option<String> {
    msgs.iter()
        .rev()
        .find(|m| m.role == Role::Assistant)
        .and_then(|m| m.content.clone())
        .filter(|s| !s.is_empty())
}

pub fn chat_to_wire(m: &ChatMessage) -> ChatMessageWire {
    let role = match m.role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
    .to_string();
    ChatMessageWire {
        role,
        content: m.content.clone(),
        tool_calls: m
            .tool_calls
            .as_ref()
            .map(|c| c.iter().map(ToolCallWire::from).collect()),
        tool_call_id: m.tool_call_id.clone(),
        name: m.name.clone(),
    }
}

pub fn wire_to_chat(m: ChatMessageWire) -> Result<ChatMessage, String> {
    let role = match m.role.as_str() {
        "system" => Role::System,
        "user" => Role::User,
        "assistant" => Role::Assistant,
        "tool" => Role::Tool,
        other => {
            return Err(format!("invalid message role: {other}"));
        }
    };
    let tool_calls = m
        .tool_calls
        .map(|v| {
            v.into_iter()
                .map(|t| ToolCall {
                    id: t.id,
                    function: FunctionCallResult {
                        name: t.name,
                        arguments: t.arguments,
                    },
                })
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty());
    Ok(ChatMessage {
        role,
        content: m.content,
        tool_calls,
        tool_call_id: m.tool_call_id,
        name: m.name,
    })
}
