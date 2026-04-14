//! 规划智能体流式入口：组装 project 规划工具集并委托 [`crate::app::execution::agent_stream`]。

use std::sync::Arc;

use agentool::Tool;
use lmkit::chat::{ChatMessage, ChatProvider, RequestPreset};

use super::agent_stream::{run_agent_turn_stream, AgentStreamError, AgentStreamTurn};
use super::planning::PlanningLoopConfig;
use super::planning_events::PlanningStreamEnvelope;
use super::tool_registry::merge_planner_tool_list;
use crate::app::project::tools::planner_project_tools;
use crate::app::state::SharedProject;

pub type PlanningStreamError = AgentStreamError;

/// 单次流式规划回合的输入。
pub struct PlanningStreamTurn<'a> {
    pub provider: &'a dyn ChatProvider,
    pub shared: SharedProject,
    pub messages: Vec<ChatMessage>,
    pub additional_tools: Vec<Arc<dyn Tool>>,
    pub config: PlanningLoopConfig,
    pub run_id: &'a str,
    pub provider_label: &'a str,
    pub model_label: &'a str,
}

pub async fn run_planning_turn_stream<E>(
    turn: PlanningStreamTurn<'_>,
    emit: E,
) -> Result<Vec<ChatMessage>, PlanningStreamError>
where
    E: FnMut(PlanningStreamEnvelope),
{
    let tools = merge_planner_tool_list(
        planner_project_tools(turn.shared.clone()),
        turn.additional_tools.as_slice(),
    );
    let inner = AgentStreamTurn {
        provider: turn.provider,
        messages: turn.messages,
        tools,
        config: turn.config,
        run_id: turn.run_id,
        provider_label: turn.provider_label,
        model_label: turn.model_label,
        preset: RequestPreset::Planning,
        agent_label: "planning",
    };
    run_agent_turn_stream(inner, emit).await
}
