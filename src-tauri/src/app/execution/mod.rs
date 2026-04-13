//! 智能体执行循环（lmkit + agentool + project 工具），后续实现。
//!
//! 规划侧：[`crate::app::project::tools::planner_project_tools`] + 其它 `agentool` 工具；
//! 执行侧：[`crate::app::project::tools::executor_project_tools`]，并可追加 [`crate::app::project::tools::preview_render_tool`]；
//! 用 [`crate::app::project::tools::definitions_for_lmkit`] 生成 [`lmkit::chat::ChatRequest`] 的 `tools` 字段。
