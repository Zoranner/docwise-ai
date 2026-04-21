//! 检查点与 UI 的桥接：更新 [`crate::app::state::ActiveContext`] 并广播 [`CHECKPOINT_CHANGED_EVENT`]。
//!
//! - **Tauri 命令** [`crate::app::commands::project_cmds::task_open_checkpoint`] 等通过 [`CheckpointBridge`] 复用同一逻辑。
//! - **执行智能体** 在流式回合中携带 [`CheckpointBridge`]，project 工具 `task_open_checkpoint` / `task_close_checkpoint` 在成功写入 DB 后同样调用。

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::app::project::dto::ReviewDto;
use crate::app::state::SharedActiveContext;

pub const CHECKPOINT_CHANGED_EVENT: &str = "docwise:checkpoint-changed";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointChangedEvent {
    pub action: &'static str,
    pub checkpoint: ReviewDto,
    /// 数据库中该任务在此操作后的 `status`（关闭时可能为 `running` 等）。
    pub task_status_after: Option<String>,
}

impl CheckpointChangedEvent {
    pub fn emit(&self, app: &AppHandle) {
        let _ = app.emit(CHECKPOINT_CHANGED_EVENT, self);
    }
}

/// 与 Tauri 前端联动：同步 ActiveContext + 发事件（需在 async 上下文中调用 `notify_*`）。
#[derive(Clone)]
pub struct CheckpointBridge {
    app: AppHandle,
    active: SharedActiveContext,
}

impl CheckpointBridge {
    pub fn new(app: AppHandle, active: SharedActiveContext) -> Self {
        Self { app, active }
    }

    pub async fn notify_opened(&self, cp: &ReviewDto) {
        {
            let mut a = self.active.0.lock().await;
            a.task_id = Some(cp.task_id.clone());
            a.checkpoint_id = Some(cp.id.clone());
        }
        CheckpointChangedEvent {
            action: "opened",
            checkpoint: cp.clone(),
            task_status_after: Some("waiting_checkpoint".to_owned()),
        }
        .emit(&self.app);
    }

    pub async fn notify_closed(&self, cp: &ReviewDto, task_status_after: Option<String>) {
        {
            let mut a = self.active.0.lock().await;
            if a.checkpoint_id.as_deref() == Some(cp.id.as_str()) {
                a.checkpoint_id = None;
            }
            a.task_id = Some(cp.task_id.clone());
        }
        CheckpointChangedEvent {
            action: "closed",
            checkpoint: cp.clone(),
            task_status_after,
        }
        .emit(&self.app);
    }
}
