//! 应用层「项目清单」：持久化在本地配置目录，与进程内 [`WorkspaceHost`](crate::app::state::WorkspaceHost) 会话分离。
//!
//! v1：JSON 文件；路径白名单仅存清单，Fs 多根裁决后续与 agentool 对齐。

use std::path::{Path, PathBuf};
use std::sync::Arc;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathAllowlistEntry {
    pub path: String,
    pub permission: String,
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppProjectRecord {
    pub id: String,
    pub name: String,
    pub workspace_path: String,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub path_allowlist: Vec<PathAllowlistEntry>,
    pub created_at: String,
    pub updated_at: String,
}

impl AppProjectRecord {
    pub fn new(
        name: String,
        workspace_path: String,
        group: Option<String>,
        path_allowlist: Vec<PathAllowlistEntry>,
    ) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            workspace_path,
            group,
            path_allowlist,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogFile {
    #[serde(default = "catalog_version")]
    pub version: u32,
    #[serde(default)]
    pub projects: Vec<AppProjectRecord>,
}

fn catalog_version() -> u32 {
    1
}

/// `%APPDATA%\Docwise\project_catalog.json`（Windows）或平台等价配置目录。
pub fn default_catalog_path() -> Result<PathBuf, String> {
    let base =
        dirs::config_dir().ok_or_else(|| "could not resolve config directory".to_owned())?;
    Ok(base.join("Docwise").join("project_catalog.json"))
}

pub fn load_catalog_sync(path: &Path) -> Result<CatalogFile, String> {
    if !path.exists() {
        return Ok(CatalogFile {
            version: 1,
            projects: vec![],
        });
    }
    let s = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&s).map_err(|e| e.to_string())
}

pub async fn persist_catalog(path: &Path, file: &CatalogFile) -> Result<(), String> {
    let json = serde_json::to_string_pretty(file).map_err(|e| e.to_string())?;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }
    tokio::fs::write(path, json)
        .await
        .map_err(|e| e.to_string())
}

pub struct CatalogStore {
    pub path: PathBuf,
    pub data: CatalogFile,
}

#[derive(Clone)]
pub struct SharedProjectCatalog {
    pub inner: Arc<Mutex<CatalogStore>>,
}

impl SharedProjectCatalog {
    pub fn new(path: PathBuf, data: CatalogFile) -> Self {
        Self {
            inner: Arc::new(Mutex::new(CatalogStore { path, data })),
        }
    }
}
