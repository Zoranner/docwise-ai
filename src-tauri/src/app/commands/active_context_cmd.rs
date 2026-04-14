use tauri::State;

use crate::app::state::{
    workspace_id_from_root, ActiveContext, SharedActiveContext, SharedProject,
};

#[tauri::command]
pub async fn active_context_get(
    state: State<'_, SharedActiveContext>,
) -> Result<ActiveContext, String> {
    Ok(state.0.lock().await.clone())
}

/// 覆盖整份上下文。若当前已打开工作区，则 `workspaceId` 必须与该工作区根路径一致（防止与 `project.db` 错位）。
#[tauri::command]
pub async fn active_context_replace(
    active: State<'_, SharedActiveContext>,
    project: State<'_, SharedProject>,
    ctx: ActiveContext,
) -> Result<(), String> {
    let open = project.0.lock().await.clone();
    if let Some(p) = open.as_ref() {
        let expected = workspace_id_from_root(p.workspace_root());
        if ctx.workspace_id != expected {
            return Err(format!(
                "activeContext.workspaceId must match open workspace (expected {expected})"
            ));
        }
    }
    *active.0.lock().await = ctx;
    Ok(())
}
