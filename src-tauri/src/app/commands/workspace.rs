use std::path::PathBuf;
use std::sync::Arc;

use serde::Serialize;
use tauri::State;
use tokio::fs;

use crate::app::project::ProjectContext;
use crate::app::state::{
    workspace_id_from_root, ActiveContext, SharedActiveContext, SharedProject,
};
use crate::app::workspace::resolve_workspace_path;

#[tauri::command]
pub async fn workspace_get_path(state: State<'_, SharedProject>) -> Result<Option<String>, String> {
    Ok(state
        .0
        .lock()
        .await
        .focused_context()
        .map(|c| c.workspace_root().to_string_lossy().into_owned()))
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSessionDto {
    pub workspace_id: String,
    pub path: String,
    pub focused: bool,
}

/// 列出当前进程内已打开的工作区（含是否前台）。
#[tauri::command]
pub async fn workspace_list_open(
    state: State<'_, SharedProject>,
) -> Result<Vec<WorkspaceSessionDto>, String> {
    let host = state.0.lock().await;
    let focused = host.focused_workspace_id.as_deref();
    let mut out: Vec<WorkspaceSessionDto> = host
        .open
        .iter()
        .map(|(wid, ctx)| WorkspaceSessionDto {
            workspace_id: wid.clone(),
            path: ctx.workspace_root().to_string_lossy().into_owned(),
            focused: focused == Some(wid.as_str()),
        })
        .collect();
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

/// 将前台切换到已打开的工作区（`workspaceId` 为规范化根路径字符串，与 `ActiveContext.workspaceId` 一致）。
#[tauri::command]
pub async fn workspace_focus(
    state: State<'_, SharedProject>,
    active: State<'_, SharedActiveContext>,
    workspace_id: String,
) -> Result<(), String> {
    let root = {
        let mut host = state.0.lock().await;
        let Some(ctx) = host.open.get(&workspace_id) else {
            return Err("workspace is not open in this session".to_owned());
        };
        let root = ctx.workspace_root().to_path_buf();
        host.focused_workspace_id = Some(workspace_id);
        root
    };
    *active.0.lock().await = ActiveContext::reset_for_workspace_root(&root);
    Ok(())
}

/// 从会话中移除一个已打开的工作区（不删磁盘）。若移除的是前台，则自动聚焦剩余集合中的任意一个或清空。
#[tauri::command]
pub async fn workspace_close(
    state: State<'_, SharedProject>,
    active: State<'_, SharedActiveContext>,
    workspace_id: String,
) -> Result<(), String> {
    let next_root = {
        let mut host = state.0.lock().await;
        host.open.remove(&workspace_id);
        if host.focused_workspace_id.as_deref() == Some(workspace_id.as_str()) {
            host.focused_workspace_id = host.open.keys().next().cloned();
        }
        host.focused_context()
            .map(|c| c.workspace_root().to_path_buf())
    };
    if let Some(r) = next_root {
        *active.0.lock().await = ActiveContext::reset_for_workspace_root(&r);
    } else {
        *active.0.lock().await = ActiveContext::default();
    }
    Ok(())
}

/// 使用已 `canonicalize` 的工作区根：插入或复用会话并设为前台，同步 [`ActiveContext`]。
pub async fn workspace_open_canonical_root(
    shared: &SharedProject,
    active: &SharedActiveContext,
    root: PathBuf,
) -> Result<(), String> {
    let wid = workspace_id_from_root(&root);
    {
        let mut host = shared.0.lock().await;
        if host.open.contains_key(&wid) {
            host.focused_workspace_id = Some(wid);
        } else {
            let ctx = ProjectContext::open(root.clone())
                .await
                .map_err(|e| format!("open project db: {e}"))?;
            host.open.insert(wid.clone(), Arc::new(ctx));
            host.focused_workspace_id = Some(wid);
        }
    }
    *active.0.lock().await = ActiveContext::reset_for_workspace_root(&root);
    Ok(())
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
    workspace_open_canonical_root(&state, &active, root).await
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
        .focused_context()
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

/// 写入工作区内 UTF-8 文本（`path` 相对工作区根；父目录不存在则创建）。已存在目录则报错。
#[tauri::command]
pub async fn workspace_write_text_file(
    state: State<'_, SharedProject>,
    path: String,
    content: String,
) -> Result<(), String> {
    let ctx = state
        .0
        .lock()
        .await
        .focused_context()
        .ok_or_else(|| "workspace not opened".to_owned())?;
    let root = ctx.workspace_root();
    let resolved = resolve_workspace_path(root, &path)?;
    if resolved.exists() && resolved.is_dir() {
        return Err("path is a directory".into());
    }
    if let Some(parent) = resolved.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("create_dir_all: {e}"))?;
    }
    fs::write(&resolved, content.as_bytes())
        .await
        .map_err(|e| format!("write file: {e}"))
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
        .focused_context()
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
