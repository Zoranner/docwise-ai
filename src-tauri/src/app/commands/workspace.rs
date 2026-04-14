use std::path::PathBuf;
use std::sync::Arc;

use serde::Serialize;
use tauri::State;
use tokio::fs;

use crate::app::project::ProjectContext;
use crate::app::state::{ActiveContext, SharedActiveContext, SharedProject};
use crate::app::workspace::resolve_workspace_path;

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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceDirEntryDto {
    pub name: String,
    pub is_dir: bool,
    pub is_file: bool,
}

/// 读取工作区内 UTF-8 文本文件（`path` 为相对工作区根的路径）。
#[tauri::command]
pub async fn workspace_read_text_file(
    state: State<'_, SharedProject>,
    path: String,
) -> Result<String, String> {
    let ctx = state
        .0
        .lock()
        .await
        .clone()
        .ok_or_else(|| "workspace not opened".to_owned())?;
    let root = ctx.workspace_root();
    let resolved = resolve_workspace_path(root, &path)?;
    if !resolved.is_file() {
        return Err("path is not a file".into());
    }
    fs::read_to_string(&resolved)
        .await
        .map_err(|e| format!("read file: {e}"))
}

/// 列出工作区内目录条目（`path` 为空表示根目录）。
#[tauri::command]
pub async fn workspace_list_directory(
    state: State<'_, SharedProject>,
    path: String,
) -> Result<Vec<WorkspaceDirEntryDto>, String> {
    let ctx = state
        .0
        .lock()
        .await
        .clone()
        .ok_or_else(|| "workspace not opened".to_owned())?;
    let root = ctx.workspace_root();
    let resolved = resolve_workspace_path(root, &path)?;
    if !resolved.is_dir() {
        return Err("path is not a directory".into());
    }
    let mut rd = fs::read_dir(&resolved)
        .await
        .map_err(|e| format!("read_dir: {e}"))?;
    let mut out = Vec::new();
    while let Some(ent) = rd
        .next_entry()
        .await
        .map_err(|e| format!("read_dir entry: {e}"))?
    {
        let meta = ent.metadata().await.map_err(|e| format!("metadata: {e}"))?;
        let name = ent.file_name().to_string_lossy().into_owned();
        out.push(WorkspaceDirEntryDto {
            name,
            is_dir: meta.is_dir(),
            is_file: meta.is_file(),
        });
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}
