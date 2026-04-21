use std::path::PathBuf;

use chrono::Utc;
use tauri::State;

use crate::app::catalog::{
    persist_catalog, AppProjectRecord, PathAllowlistEntry, SharedProjectCatalog,
};
use crate::app::commands::workspace::workspace_open_canonical_root;
use crate::app::state::{SharedActiveContext, SharedProject};

#[tauri::command]
pub async fn app_project_list(
    catalog: State<'_, SharedProjectCatalog>,
) -> Result<Vec<AppProjectRecord>, String> {
    let g = catalog.inner.lock().await;
    Ok(g.data.projects.clone())
}

#[tauri::command]
pub async fn app_project_add(
    catalog: State<'_, SharedProjectCatalog>,
    name: String,
    workspace_path: String,
    group: Option<String>,
    path_allowlist: Option<Vec<PathAllowlistEntry>>,
) -> Result<AppProjectRecord, String> {
    let root = PathBuf::from(workspace_path.trim());
    if !root.is_dir() {
        return Err("workspace path is not a directory".to_owned());
    }
    let _ = root
        .canonicalize()
        .map_err(|e| format!("canonicalize workspace: {e}"))?;

    let rec = AppProjectRecord::new(
        name.trim().to_owned(),
        root.to_string_lossy().into_owned(),
        group.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()),
        path_allowlist.unwrap_or_default(),
    );

    let mut g = catalog.inner.lock().await;
    g.data.projects.push(rec.clone());
    persist_catalog(&g.path, &g.data).await?;
    Ok(rec)
}

#[tauri::command]
pub async fn app_project_remove(
    catalog: State<'_, SharedProjectCatalog>,
    id: String,
) -> Result<(), String> {
    let mut g = catalog.inner.lock().await;
    let n = g.data.projects.len();
    g.data.projects.retain(|p| p.id != id);
    if g.data.projects.len() == n {
        return Err("project id not found".to_owned());
    }
    persist_catalog(&g.path, &g.data).await
}

#[tauri::command]
pub async fn app_project_update(
    catalog: State<'_, SharedProjectCatalog>,
    id: String,
    name: String,
    workspace_path: String,
    group: Option<String>,
    path_allowlist: Option<Vec<PathAllowlistEntry>>,
) -> Result<AppProjectRecord, String> {
    let root = PathBuf::from(workspace_path.trim());
    if !root.is_dir() {
        return Err("workspace path is not a directory".to_owned());
    }
    let _ = root
        .canonicalize()
        .map_err(|e| format!("canonicalize workspace: {e}"))?;

    let mut g = catalog.inner.lock().await;
    let Some(p) = g.data.projects.iter_mut().find(|p| p.id == id) else {
        return Err("project id not found".to_owned());
    };
    p.name = name.trim().to_owned();
    p.workspace_path = root.to_string_lossy().into_owned();
    p.group = group.map(|s| s.trim().to_owned()).filter(|s| !s.is_empty());
    p.path_allowlist = path_allowlist.unwrap_or_default();
    p.updated_at = Utc::now().to_rfc3339();
    let out = p.clone();
    persist_catalog(&g.path, &g.data).await?;
    Ok(out)
}

/// 按清单 id 打开工作区（路径来自清单）；不修改清单内容。
#[tauri::command]
pub async fn app_project_open_workspace(
    catalog: State<'_, SharedProjectCatalog>,
    workspaces: State<'_, SharedProject>,
    active: State<'_, SharedActiveContext>,
    project_id: String,
) -> Result<(), String> {
    let path = {
        let g = catalog.inner.lock().await;
        g.data
            .projects
            .iter()
            .find(|p| p.id == project_id)
            .map(|p| p.workspace_path.clone())
            .ok_or_else(|| "project id not found".to_owned())?
    };
    let root = PathBuf::from(path);
    if !root.is_dir() {
        return Err("workspace path is not a directory".to_owned());
    }
    let root = root
        .canonicalize()
        .map_err(|e| format!("canonicalize workspace: {e}"))?;
    workspace_open_canonical_root(&workspaces, &active, root).await
}
