//! 文档执行智能体流式入口：project 执行工具集 + 可选 [`preview_render`](crate::app::project::tools::preview_render_tool)。

use std::sync::Arc;

use agentool::Tool;
use lmkit::chat::{ChatMessage, ChatProvider, RequestPreset};

use super::agent_stream::{run_agent_turn_stream, AgentStreamError, AgentStreamTurn};
use super::planning::PlanningLoopConfig;
use super::planning_events::PlanningStreamEnvelope;
use super::tool_registry::merge_planner_tool_list;
use crate::app::checkpoint::CheckpointBridge;
use crate::app::project::tools::{executor_project_tools, preview_render_tool};
use crate::app::state::SharedProject;

pub type ExecutionStreamError = AgentStreamError;

pub struct ExecutionStreamTurn<'a> {
    pub provider: &'a dyn ChatProvider,
    pub shared: SharedProject,
    pub messages: Vec<ChatMessage>,
    pub additional_tools: Vec<Arc<dyn Tool>>,
    pub config: PlanningLoopConfig,
    pub run_id: &'a str,
    pub provider_label: &'a str,
    pub model_label: &'a str,
    pub include_preview_tool: bool,
    /// 非 None 时，`task_open_checkpoint` / `task_close_checkpoint` 工具与 IPC 行为一致（事件 + ActiveContext）。
    pub checkpoint_bridge: Option<Arc<CheckpointBridge>>,
}

pub async fn run_execution_turn_stream<E>(
    turn: ExecutionStreamTurn<'_>,
    emit: E,
) -> Result<Vec<ChatMessage>, ExecutionStreamError>
where
    E: FnMut(PlanningStreamEnvelope),
{
    let mut tools = merge_planner_tool_list(
        executor_project_tools(turn.shared.clone(), turn.checkpoint_bridge.clone()),
        turn.additional_tools.as_slice(),
    );
    if turn.include_preview_tool {
        tools.push(preview_render_tool());
    }
    let inner = AgentStreamTurn {
        provider: turn.provider,
        messages: turn.messages,
        tools,
        config: turn.config,
        run_id: turn.run_id,
        provider_label: turn.provider_label,
        model_label: turn.model_label,
        preset: RequestPreset::Execution,
        agent_label: "execution",
    };
    run_agent_turn_stream(inner, emit).await
}
