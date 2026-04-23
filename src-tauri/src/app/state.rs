use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::app::project::ProjectContext;

/// 已打开的工作区集合与当前前台（多项目并行会话）。
#[derive(Clone, Default)]
pub struct WorkspaceHost {
    pub open: HashMap<String, Arc<ProjectContext>>,
    pub focused_workspace_id: Option<String>,
}

impl WorkspaceHost {
    pub fn focused_context(&self) -> Option<Arc<ProjectContext>> {
        let id = self.focused_workspace_id.as_ref()?;
        self.open.get(id).cloned()
    }
}

/// 全局工作区宿主（[`tauri::State`]）。
#[derive(Clone)]
pub struct SharedProject(pub Arc<Mutex<WorkspaceHost>>);

impl SharedProject {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(WorkspaceHost::default())))
    }
}

impl Default for SharedProject {
    fn default() -> Self {
        Self::new()
    }
}

/// 与 [`product-design.md`](../../docs/product-design.md) 中 **ActiveContext** 一致：侧栏、监看区、对话共享的项目焦点上下文（内存态）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveContext {
    /// 当前工作区标识；v1 使用规范化后的工作区根路径（统一 `/` 分隔符）。
    pub workspace_id: String,
    pub project_id: Option<String>,
    pub blueprint_id: Option<String>,
    pub task_id: Option<String>,
    pub review_id: Option<String>,
    pub output_id: Option<String>,
}

impl ActiveContext {
    /// 打开工作区后重置「焦点」字段，仅保留 `workspace_id`。
    pub fn reset_for_workspace_root(root: &Path) -> Self {
        Self {
            workspace_id: workspace_id_from_root(root),
            project_id: None,
            blueprint_id: None,
            task_id: None,
            review_id: None,
            output_id: None,
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::ActiveContext;

    #[test]
    fn reset_for_workspace_root_clears_focus_fields_but_keeps_workspace() {
        let ctx = ActiveContext::reset_for_workspace_root(Path::new("E:/demo"));

        assert_eq!(ctx.workspace_id, "E:/demo");
        assert_eq!(ctx.project_id, None);
        assert_eq!(ctx.blueprint_id, None);
        assert_eq!(ctx.task_id, None);
        assert_eq!(ctx.review_id, None);
        assert_eq!(ctx.output_id, None);
    }
}
