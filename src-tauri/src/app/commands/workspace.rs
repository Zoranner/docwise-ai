use std::path::PathBuf;
use std::sync::Arc;

use tauri::State;

use crate::app::project::ProjectContext;
use crate::app::state::{ActiveContext, SharedActiveContext, SharedProject};

#[tauri::command]
pub async fn workspace_get_path(state: State<'_, SharedProject>) -> Result<Option<String>, String> {
    Ok(state
        .0
        .lock()
        .await
        .as_ref()
        .map(|c| c.workspace_root().to_string_lossy().into_owned()))
}

#[tauri::command]
pub async fn workspace_open(
    state: State<'_, SharedProject>,
    active: State<'_, SharedActiveContext>,
    path: String,
) -> Result<(), String> {
    let root = PathBuf::from(path);
    if !root.is_dir() {
        return Err("workspace path is not a directory".to_owned());
    }
    let root = root
        .canonicalize()
        .map_err(|e| format!("canonicalize workspace: {e}"))?;
    let ctx = ProjectContext::open(root.clone())
        .await
        .map_err(|e| format!("open project db: {e}"))?;
    let mut guard = state.0.lock().await;
    *guard = Some(Arc::new(ctx));
    drop(guard);
    *active.0.lock().await = ActiveContext::reset_for_workspace_root(&root);
    Ok(())
}
