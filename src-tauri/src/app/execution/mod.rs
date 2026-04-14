//! 智能体执行循环（lmkit + agentool + project 工具）。
//!
//! - **规划（流式）**：[`run_planning_turn_stream`] — [`RequestPreset::Planning`]，事件 [`PLANNING_AGENT_EVENT`]。
//! - **执行（流式）**：[`run_execution_turn_stream`] — [`RequestPreset::Execution`]，可选 `preview_render`，事件 [`EXECUTION_AGENT_EVENT`]。
//! - **共用载荷**：[`PlanningStreamEnvelope`] / [`PlanningStreamEvent`]（`turn_started.agent` 区分角色）。
//! - **阻塞规划**：[`planning::run_planning_turn`]。

mod agent_stream;
pub mod agentool_bundles;
mod execution_stream;
mod llm_retry;
mod planning_core;
mod planning_events;
mod planning_stream;

pub mod planning;
pub mod tool_registry;

pub use agent_stream::{run_agent_turn_stream, AgentStreamError, AgentStreamTurn};
pub use execution_stream::{run_execution_turn_stream, ExecutionStreamError, ExecutionStreamTurn};
pub use planning::{run_planning_turn, PlanningAgentError, PlanningLoopConfig, PlanningTurnResult};
pub use planning_events::{
    wire_to_chat, ChatMessageWire, PlanningStreamEnvelope, PlanningStreamEvent, ToolCallWire,
    EXECUTION_AGENT_EVENT, PLANNING_AGENT_EVENT,
};
pub use planning_stream::{run_planning_turn_stream, PlanningStreamError, PlanningStreamTurn};
pub use tool_registry::{merge_planner_tool_list, tools_by_name};
