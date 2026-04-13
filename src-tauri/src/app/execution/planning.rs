//! 规划对话智能体：非流式多轮 `complete`，[`RequestPreset::Planning`]，工具调用写回 `ChatMessage`。
//!
//! 工具集默认 [`crate::app::project::tools::planner_project_tools`]，可通过 `additional_tools` 追加 agentool（如 `file_read`），同名以 project 为准。

use std::sync::Arc;

use agentool::Tool;
use lmkit::chat::{ChatMessage, ChatProvider, ChatRequest, RequestPreset, Role, ToolChoice};
use serde_json::{json, Value};

use super::tool_registry::{merge_planner_tool_list, tools_by_name};
use crate::app::project::tools::{definitions_for_lmkit, planner_project_tools};
use crate::app::state::SharedProject;

/// 规划循环配置。
#[derive(Debug, Clone)]
pub struct PlanningLoopConfig {
    /// 单次用户回合内，最多执行多少轮「模型 → tool → 模型」。
    pub max_tool_rounds: u32,
}

impl Default for PlanningLoopConfig {
    fn default() -> Self {
        Self {
            max_tool_rounds: 24,
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
            Self::Lmkit(e) => write!(f, "{e}"),
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

fn parse_tool_arguments(raw: &str) -> Value {
    let t = raw.trim();
    if t.is_empty() {
        return json!({});
    }
    serde_json::from_str(t).unwrap_or_else(|_| json!({ "_raw": raw }))
}

async fn dispatch_tool(
    registry: &std::collections::HashMap<String, Arc<dyn Tool>>,
    name: &str,
    args: Value,
) -> Value {
    match registry.get(name) {
        Some(tool) => match tool.execute(args).await {
            Ok(v) => v,
            Err(e) => json!({ "error": { "code": e.code, "message": e.message } }),
        },
        None => json!({
            "error": {
                "code": "unknown_tool",
                "message": format!("no tool registered named {name:?}")
            }
        }),
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

        let resp = provider.complete(&req).await?;

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
