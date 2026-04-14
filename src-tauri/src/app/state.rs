use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::app::project::ProjectContext;

/// 当前打开的 `ProjectContext`（工作区 `.agent/project.db`）。
#[derive(Clone)]
pub struct SharedProject(pub Arc<Mutex<Option<Arc<ProjectContext>>>>);

impl SharedProject {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }
}

impl Default for SharedProject {
    fn default() -> Self {
        Self::new()
    }
}

/// 与 [`docwise-design.md`](../../docs/docwise-design.md) 中 **ActiveContext** 一致：侧栏、看板、对话共享的导航上下文（内存态）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveContext {
    /// 当前工作区标识；v1 使用规范化后的工作区根路径（统一 `/` 分隔符）。
    pub workspace_id: String,
    pub file_path: Option<String>,
    pub blueprint_id: Option<String>,
    pub task_id: Option<String>,
    pub run_id: Option<String>,
    pub checkpoint_id: Option<String>,
}

impl ActiveContext {
    /// 打开工作区后重置「焦点」字段，仅保留 `workspace_id`。
    pub fn reset_for_workspace_root(root: &Path) -> Self {
        Self {
            workspace_id: workspace_id_from_root(root),
            file_path: None,
            blueprint_id: None,
            task_id: None,
            run_id: None,
            checkpoint_id: None,
        }
    }
}

/// 将本地路径转为前端可用的稳定 `workspaceId` 字符串。
pub fn workspace_id_from_root(root: &Path) -> String {
    root.to_string_lossy().replace('\\', "/")
}

/// 全局 ActiveContext（[`tauri::State`]）。
#[derive(Clone)]
pub struct SharedActiveContext(pub Arc<Mutex<ActiveContext>>);

impl SharedActiveContext {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(ActiveContext::default())))
    }
}

impl Default for SharedActiveContext {
    fn default() -> Self {
        Self::new()
    }
}
