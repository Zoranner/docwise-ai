use crate::app::preview::{ComrakStubBackend, RenderPreviewResult};

#[tauri::command]
pub fn preview_render(content: String, snapshot_id: Option<String>) -> RenderPreviewResult {
    let id = snapshot_id.unwrap_or_else(|| "editor-buffer".to_owned());
    ComrakStubBackend::render(&content, id)
}
