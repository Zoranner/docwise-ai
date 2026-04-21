mod app;

use tauri::Manager;

pub use app::catalog::SharedProjectCatalog;
pub use app::checkpoint::{CheckpointBridge, CHECKPOINT_CHANGED_EVENT};
pub use app::execution::{
    merge_planner_tool_list, run_agent_turn_stream, run_execution_turn_stream, run_planning_turn,
    run_planning_turn_stream, tools_by_name, AgentStreamError, AgentStreamTurn, ChatMessageWire,
    ExecutionStreamError, ExecutionStreamTurn, PlanningAgentError, PlanningLoopConfig,
    PlanningStreamEnvelope, PlanningStreamError, PlanningStreamEvent, PlanningStreamTurn,
    PlanningTurnResult, ToolCallWire, EXECUTION_AGENT_EVENT, PLANNING_AGENT_EVENT,
};
pub use app::project::tools::{
    definitions_for_lmkit, executor_project_tools, planner_project_tools, preview_render_tool,
};
pub use app::state::{ActiveContext, SharedActiveContext, SharedProject, WorkspaceHost};

use app::catalog::{self, CatalogFile};
use app::commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            let path = catalog::default_catalog_path().map_err(std::io::Error::other)?;
            let data = catalog::load_catalog_sync(&path).unwrap_or_else(|e| {
                eprintln!("docwise: project catalog invalid ({e}), using empty catalog");
                CatalogFile {
                    version: 1,
                    projects: vec![],
                }
            });
            _app.manage(catalog::SharedProjectCatalog::new(path, data));
            Ok(())
        })
        .manage(SharedProject::default())
        .manage(SharedActiveContext::default())
        .invoke_handler(tauri::generate_handler![
            commands::active_context_get,
            commands::active_context_replace,
            commands::execution_agent_turn_stream,
            commands::planning_agent_turn_stream,
            commands::workspace_get_path,
            commands::workspace_list_open,
            commands::workspace_focus,
            commands::workspace_close,
            commands::workspace_open,
            commands::app_project_list,
            commands::app_project_add,
            commands::app_project_remove,
            commands::app_project_update,
            commands::app_project_open_workspace,
            commands::session_workspaces_overview,
            commands::workspace_read_text_file,
            commands::workspace_write_text_file,
            commands::workspace_list_directory,
            commands::preview_render,
            commands::blueprint_create,
            commands::blueprint_get,
            commands::blueprint_list,
            commands::blueprint_update,
            commands::blueprint_set_status,
            commands::blueprint_item_add,
            commands::blueprint_item_update,
            commands::blueprint_item_remove,
            commands::task_create,
            commands::task_list,
            commands::task_get,
            commands::task_get_tree,
            commands::task_update,
            commands::task_delete,
            commands::task_start_run,
            commands::task_end_run,
            commands::task_append_step,
            commands::task_update_step,
            commands::task_open_checkpoint,
            commands::task_close_checkpoint,
            commands::task_acquire_lock,
            commands::task_release_lock,
            commands::task_add_artifact,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Docwise");
}
