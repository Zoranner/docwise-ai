use std::sync::Arc;

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
