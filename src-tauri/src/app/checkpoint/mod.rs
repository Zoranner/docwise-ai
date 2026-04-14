//! 检查点与 UI 的桥接：经 Tauri IPC 打开/关闭检查点时广播事件，并同步 [`crate::app::state::ActiveContext`]。
//!
//! 智能体在宿主内通过 project 工具直接调用 `ops` 时当前**不会**触发事件（无 [`AppHandle`]）；仅 `task_open_checkpoint` / `task_close_checkpoint` 命令路径会广播。

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::app::project::dto::CheckpointDto;

pub const CHECKPOINT_CHANGED_EVENT: &str = "docwise:checkpoint-changed";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckpointChangedEvent {
    pub action: &'static str,
    pub checkpoint: CheckpointDto,
    /// 数据库中该任务在此操作后的 `status`（关闭时可能为 `running` 等）。
    pub task_status_after: Option<String>,
}

impl CheckpointChangedEvent {
    pub fn emit(&self, app: &AppHandle) {
        let _ = app.emit(CHECKPOINT_CHANGED_EVENT, self);
    }
}
