//! 规划对话智能体：非流式多轮 `complete`，[`RequestPreset::Planning`]，工具调用写回 `ChatMessage`。
//!
//! 工具集默认 [`crate::app::project::tools::planner_project_tools`]，可通过 `additional_tools` 追加 agentool（如 `file_read`），同名以 project 为准。

use std::sync::Arc;

use super::planning_core::{dispatch_tool, parse_tool_arguments};
use super::tool_registry::{merge_planner_tool_list, tools_by_name};
use crate::app::project::tools::{definitions_for_lmkit, planner_project_tools};
use crate::app::state::SharedProject;
use agentool::Tool;
use lmkit::chat::{ChatMessage, ChatProvider, ChatRequest, RequestPreset, Role, ToolChoice};

/// 规划循环配置。
#[derive(Debug, Clone)]
pub struct PlanningLoopConfig {
    /// 单次用户回合内，最多执行多少轮「模型 → tool → 模型」。
    pub max_tool_rounds: u32,
    /// 单次「模型请求」在可重试错误时的**额外**尝试次数（不含首次）。例如 `3` 表示最多共 4 次 HTTP/SSE。
    pub llm_max_retries: u32,
    pub llm_retry_base_delay_ms: u64,
    pub llm_retry_max_delay_ms: u64,
}

impl Default for PlanningLoopConfig {
    fn default() -> Self {
        Self {
            max_tool_rounds: 24,
            llm_max_retries: 3,
            llm_retry_base_delay_ms: 500,
            llm_retry_max_delay_ms: 30_000,
        }
    }
}

impl PlanningLoopConfig {
    /// 流式 / 命令入口：未提供的字段用 [`PlanningLoopConfig::default`]。
    pub fn for_agent_command(
        max_tool_rounds: Option<u32>,
        llm_max_retries: Option<u32>,
        llm_retry_base_delay_ms: Option<u64>,
        llm_retry_max_delay_ms: Option<u64>,
    ) -> Self {
        let d = Self::default();
        Self {
            max_tool_rounds: max_tool_rounds.unwrap_or(d.max_tool_rounds),
            llm_max_retries: llm_max_retries.unwrap_or(d.llm_max_retries),
            llm_retry_base_delay_ms: llm_retry_base_delay_ms.unwrap_or(d.llm_retry_base_delay_ms),
            llm_retry_max_delay_ms: llm_retry_max_delay_ms.unwrap_or(d.llm_retry_max_delay_ms),
        }
    }
}

/// 一次 `run_planning_turn` 的输出：完整消息轨迹（含 `tool` 角色）与最终 assistant 文本。
#[derive(Debug, Clone)]
pub struct PlanningTurnResult {
    pub messages: Vec<ChatMessage>,
    pub last_assistant_text: Option<String>,
    pub tool_rounds_completed: u32,
}

#[derive(Debug)]
pub enum PlanningAgentError {
    Lmkit(lmkit::Error),
    MaxToolRounds {
        limit: u32,
        partial_messages: Vec<ChatMessage>,
    },
}

impl std::fmt::Display for PlanningAgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lmkit(e) => write!(f, "{}", e.format_diagnostic()),
            Self::MaxToolRounds { limit, .. } => {
                write!(f, "planning agent exceeded max tool rounds ({limit})")
            }
        }
    }
}

impl std::error::Error for PlanningAgentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Lmkit(e) => Some(e),
            Self::MaxToolRounds { .. } => None,
        }
    }
}

impl From<lmkit::Error> for PlanningAgentError {
    fn from(e: lmkit::Error) -> Self {
        Self::Lmkit(e)
    }
}

/// 运行规划智能体的一个用户回合：在 `messages` 基础上追加 assistant / tool 消息，直到模型不再请求工具或达到轮次上限。
///
/// `messages` 通常已含 `system`（可选）与最新 `user`；本函数会原地追加后续轮次。
pub async fn run_planning_turn(
    provider: &dyn ChatProvider,
    shared: SharedProject,
    mut messages: Vec<ChatMessage>,
    additional_tools: &[Arc<dyn Tool>],
    config: PlanningLoopConfig,
) -> Result<PlanningTurnResult, PlanningAgentError> {
    let tools = merge_planner_tool_list(planner_project_tools(shared), additional_tools);
    let tool_defs = definitions_for_lmkit(&tools);
    let registry = tools_by_name(&tools);

    let mut tool_rounds_completed = 0u32;

    loop {
        let req = ChatRequest {
            messages: messages.clone(),
            tools: Some(tool_defs.clone()),
            tool_choice: Some(ToolChoice::Auto),
            preset: Some(RequestPreset::Planning),
            ..Default::default()
        };

        let max_attempts = config.llm_max_retries.saturating_add(1).max(1);
        let mut resp_opt = None;
        for attempt in 0..max_attempts {
            if attempt > 0 {
                let delay = super::llm_retry::backoff_delay_ms(
                    attempt - 1,
                    config.llm_retry_base_delay_ms,
                    config.llm_retry_max_delay_ms,
                );
                super::llm_retry::sleep_delay_ms(delay).await;
            }
            match provider.complete(&req).await {
                Ok(r) => {
                    resp_opt = Some(r);
                    break;
                }
                Err(e) => {
                    if e.is_retryable() && attempt + 1 < max_attempts {
                        continue;
                    }
                    return Err(PlanningAgentError::from(e));
                }
            }
        }
        let resp = resp_opt.expect("at least one attempt");

        let assistant_msg = ChatMessage {
            role: Role::Assistant,
            content: resp.content.clone(),
            tool_calls: resp.tool_calls.clone(),
            tool_call_id: None,
            name: None,
        };
        messages.push(assistant_msg);

        let Some(calls) = resp.tool_calls.filter(|c| !c.is_empty()) else {
            return Ok(PlanningTurnResult {
                messages,
                last_assistant_text: resp.content,
                tool_rounds_completed,
            });
        };

        tool_rounds_completed += 1;
        if tool_rounds_completed > config.max_tool_rounds {
            return Err(PlanningAgentError::MaxToolRounds {
                limit: config.max_tool_rounds,
                partial_messages: messages,
            });
        }

        for tc in calls {
            let args = parse_tool_arguments(&tc.function.arguments);
            let out = dispatch_tool(&registry, &tc.function.name, args).await;
            let payload = serde_json::to_string(&out).unwrap_or_else(|_| out.to_string());
            messages.push(ChatMessage::tool_with_name(
                &tc.id,
                &tc.function.name,
                payload,
            ));
        }
    }
}
