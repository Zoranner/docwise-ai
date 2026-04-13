use std::path::PathBuf;
use std::sync::Arc;

use tauri::State;

use crate::app::preview::{ComrakStubBackend, RenderPreviewResult};
use crate::app::project::ProjectContext;
use crate::app::state::SharedProject;

#[tauri::command]
pub async fn workspace_open(state: State<'_, SharedProject>, path: String) -> Result<(), String> {
    let root = PathBuf::from(path);
    if !root.is_dir() {
        return Err("workspace path is not a directory".to_owned());
    }
    let ctx = ProjectContext::open(root)
        .await
        .map_err(|e| format!("open project db: {e}"))?;
    let mut guard = state.0.lock().await;
    *guard = Some(Arc::new(ctx));
    Ok(())
}

#[tauri::command]
pub fn preview_render(content: String, snapshot_id: Option<String>) -> RenderPreviewResult {
    let id = snapshot_id.unwrap_or_else(|| "editor-buffer".to_owned());
    ComrakStubBackend::render(&content, id)
}
