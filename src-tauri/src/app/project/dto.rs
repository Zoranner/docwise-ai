use std::collections::BTreeMap;

use serde::Serialize;

use super::entity::{
    blueprint, blueprint_item, output, path_lock, review, task, task_run, task_step,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlueprintDto {
    pub id: String,
    pub title: String,
    pub status: String,
    pub goal: String,
    pub audience: String,
    pub style_guide: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<blueprint::Model> for BlueprintDto {
    fn from(m: blueprint::Model) -> Self {
        Self {
            id: m.id,
            title: m.title,
            status: m.status,
            goal: m.goal,
            audience: m.audience,
            style_guide: m.style_guide,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlueprintItemDto {
    pub id: String,
    pub blueprint_id: String,
    pub seq: i32,
    pub file_path: String,
    pub title: String,
    pub audience: String,
    pub goal: String,
    pub must_cover: String,
    pub constraints: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<blueprint_item::Model> for BlueprintItemDto {
    fn from(m: blueprint_item::Model) -> Self {
        Self {
            id: m.id,
            blueprint_id: m.blueprint_id,
            seq: m.seq,
            file_path: m.file_path,
            title: m.title,
            audience: m.audience,
            goal: m.goal,
            must_cover: m.must_cover,
            constraints: m.constraints,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlueprintDetailDto {
    pub blueprint: BlueprintDto,
    pub items: Vec<BlueprintItemDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDto {
    pub id: String,
    pub blueprint_item_id: Option<String>,
    pub parent_id: Option<String>,
    pub conversation_ref: String,
    pub title: String,
    pub goal: String,
    pub acceptance: String,
    pub status: String,
    pub priority: i32,
    pub blocked_reason: String,
    pub tags: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<task::Model> for TaskDto {
    fn from(m: task::Model) -> Self {
        Self {
            id: m.id,
            blueprint_item_id: m.blueprint_item_id,
            parent_id: m.parent_id,
            conversation_ref: m.conversation_ref,
            title: m.title,
            goal: m.goal,
            acceptance: m.acceptance,
            status: m.status,
            priority: m.priority,
            blocked_reason: m.blocked_reason,
            tags: m.tags,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStepDto {
    pub id: String,
    pub task_id: String,
    pub seq: i32,
    pub title: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<task_step::Model> for TaskStepDto {
    fn from(m: task_step::Model) -> Self {
        Self {
            id: m.id,
            task_id: m.task_id,
            seq: m.seq,
            title: m.title,
            status: m.status,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRunDto {
    pub id: String,
    pub task_id: String,
    pub status: String,
    pub summary: String,
    pub error: String,
    pub started_at: String,
    pub ended_at: Option<String>,
}

impl From<task_run::Model> for TaskRunDto {
    fn from(m: task_run::Model) -> Self {
        Self {
            id: m.id,
            task_id: m.task_id,
            status: m.status,
            summary: m.summary,
            error: m.error,
            started_at: m.started_at,
            ended_at: m.ended_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewDto {
    pub id: String,
    pub task_id: String,
    pub status: String,
    pub conversation_ref: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<review::Model> for ReviewDto {
    fn from(m: review::Model) -> Self {
        Self {
            id: m.id,
            task_id: m.task_id,
            status: m.status,
            conversation_ref: m.conversation_ref,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PathLockDto {
    pub id: String,
    pub task_id: String,
    pub path: String,
    pub expires_at: Option<String>,
    pub created_at: String,
}

impl From<path_lock::Model> for PathLockDto {
    fn from(m: path_lock::Model) -> Self {
        Self {
            id: m.id,
            task_id: m.task_id,
            path: m.path,
            expires_at: m.expires_at,
            created_at: m.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputDto {
    pub id: String,
    pub task_id: String,
    pub run_id: Option<String>,
    pub kind: String,
    pub path: String,
    pub content: String,
    pub created_at: String,
}

impl From<output::Model> for OutputDto {
    fn from(m: output::Model) -> Self {
        Self {
            id: m.id,
            task_id: m.task_id,
            run_id: m.run_id,
            kind: m.kind,
            path: m.path,
            content: m.content,
            created_at: m.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDetailDto {
    pub task: TaskDto,
    pub steps: Vec<TaskStepDto>,
    pub runs: Vec<TaskRunDto>,
    pub reviews: Vec<ReviewDto>,
    pub path_locks: Vec<PathLockDto>,
    pub outputs: Vec<OutputDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskSubtreeSummaryDto {
    pub total: usize,
    pub by_status: BTreeMap<String, u32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskTreeNodeDto {
    pub task: TaskDto,
    pub children: Vec<TaskTreeNodeDto>,
    pub summary: TaskSubtreeSummaryDto,
}
