//! 智能体执行循环（lmkit + agentool + project 工具）。
//!
//! - **规划**：[`planning`] — [`RequestPreset::Planning`]、project 蓝图/任务工具 + 可选 agentool。
//! - **执行**（后续）：[`crate::app::project::tools::executor_project_tools`]，[`RequestPreset::Execution`]，可追加 [`crate::app::project::tools::preview_render_tool`]。

pub mod planning;
pub mod tool_registry;

pub use planning::{run_planning_turn, PlanningAgentError, PlanningLoopConfig, PlanningTurnResult};
pub use tool_registry::{merge_planner_tool_list, tools_by_name};
