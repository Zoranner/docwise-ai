//! Docwise 内置 project / preview 工具：实现 [`agentool::Tool`]，供执行循环注册到 lmkit。
//!
//! 参数 JSON 使用 **snake_case**（与 `agentool` 其它工具一致）；Tauri 命令仍用 camelCase（见 `params.rs`）。

use std::sync::Arc;

use agentool::{Tool, ToolError, ToolResult};
use lmkit::chat::ToolDefinition;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use serde_json::{json, Value};

use super::ops;
use super::params::{BlueprintItemAddParams, BlueprintItemUpdateParams, TaskUpdateParams};
use super::ProjectContext;
use crate::app::preview::ComrakStubBackend;
use crate::app::state::SharedProject;

macro_rules! project_tool {
    (
        $struct:ident, $params:ident,
        { $( $pf:ident : $pt:ty ),* $(,)? },
        $tool_name:literal,
        $desc:literal,
        $schema:tt,
        $exec:ident
    ) => {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub(crate) struct $params {
            $( pub(crate) $pf : $pt ),*
        }

        pub(crate) struct $struct {
            state: SharedProject,
        }

        impl $struct {
            pub(crate) fn new(state: SharedProject) -> Self {
                Self { state }
            }
        }

        #[async_trait::async_trait]
        impl Tool for $struct {
            fn name(&self) -> &str {
                $tool_name
            }

            fn description(&self) -> &str {
                $desc
            }

            fn schema(&self) -> Value {
                json!($schema)
            }

            async fn execute(&self, params: Value) -> ToolResult {
                let p: $params = serde_json::from_value(params).map_err(invalid_params)?;
                let ctx = require_ctx(&self.state).await?;
                $exec(&ctx.db, p).await
            }
        }
    };
}

async fn require_ctx(state: &SharedProject) -> Result<Arc<ProjectContext>, ToolError> {
    state.0.lock().await.clone().ok_or_else(|| ToolError {
        code: "workspace_not_open".into(),
        message: "Open a workspace before using project tools.".into(),
    })
}

fn invalid_params(e: serde_json::Error) -> ToolError {
    ToolError {
        code: "invalid_params".into(),
        message: e.to_string(),
    }
}

fn db_tool_err(e: sea_orm::DbErr) -> ToolError {
    ToolError {
        code: "db_error".into(),
        message: e.to_string(),
    }
}

fn to_json<T: serde::Serialize>(v: T) -> ToolResult {
    serde_json::to_value(v).map_err(|e| ToolError {
        code: "serialize_error".into(),
        message: e.to_string(),
    })
}

// --- Blueprint tools (struct + impl) ---

project_tool!(
    BlueprintCreateTool,
    BlueprintCreateParams,
    {
        title: String,
        goal: Option<String>,
        audience: Option<String>,
        style_guide: Option<String>
    },
    "blueprint_create",
    "Create a blueprint draft in workspace project.db (status draft).",
    {
        "type": "object",
        "properties": {
            "title": { "type": "string", "description": "Blueprint title (required)" },
            "goal": { "type": "string", "description": "Optional goal text" },
            "audience": { "type": "string", "description": "Optional audience" },
            "style_guide": { "type": "string", "description": "Optional JSON or prose style constraints" }
        },
        "required": ["title"]
    },
    exec_blueprint_create
);

project_tool!(
    BlueprintGetTool,
    BlueprintGetParams,
    { id: String },
    "blueprint_get",
    "Load blueprint by id including all blueprint_item rows ordered by seq.",
    {
        "type": "object",
        "properties": {
            "id": { "type": "string", "description": "Blueprint id" }
        },
        "required": ["id"]
    },
    exec_blueprint_get
);

project_tool!(
    BlueprintListTool,
    BlueprintListParams,
    { status: Option<String> },
    "blueprint_list",
    "List blueprints, optionally filtered by status (e.g. draft, approved, active, archived).",
    {
        "type": "object",
        "properties": {
            "status": { "type": "string", "description": "Optional status filter" }
        }
    },
    exec_blueprint_list
);

project_tool!(
    BlueprintUpdateTool,
    BlueprintUpdateParams,
    {
        id: String,
        title: Option<String>,
        goal: Option<String>,
        audience: Option<String>,
        style_guide: Option<String>
    },
    "blueprint_update",
    "Patch blueprint fields; only provided fields are updated.",
    {
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "title": { "type": "string" },
            "goal": { "type": "string" },
            "audience": { "type": "string" },
            "style_guide": { "type": "string" }
        },
        "required": ["id"]
    },
    exec_blueprint_update
);

project_tool!(
    BlueprintSetStatusTool,
    BlueprintSetStatusParams,
    { id: String, action: String },
    "blueprint_set_status",
    "Transition blueprint status: action one of approve, archive, supersede, activate (valid combinations depend on current status).",
    {
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "action": {
                "type": "string",
                "description": "approve | archive | supersede | activate"
            }
        },
        "required": ["id", "action"]
    },
    exec_blueprint_set_status
);

project_tool!(
    BlueprintItemAddTool,
    BlueprintItemAddSnake,
    {
        blueprint_id: String,
        file_path: String,
        title: String,
        audience: Option<String>,
        goal: Option<String>,
        must_cover: Option<String>,
        constraints: Option<String>
    },
    "blueprint_item_add",
    "Append a document requirement row to a blueprint (auto seq).",
    {
        "type": "object",
        "properties": {
            "blueprint_id": { "type": "string" },
            "file_path": { "type": "string", "description": "Target path for the document" },
            "title": { "type": "string" },
            "audience": { "type": "string" },
            "goal": { "type": "string" },
            "must_cover": { "type": "string", "description": "JSON array as string, default []" },
            "constraints": { "type": "string", "description": "JSON array as string, default []" }
        },
        "required": ["blueprint_id", "file_path", "title"]
    },
    exec_blueprint_item_add
);

project_tool!(
    BlueprintItemUpdateTool,
    BlueprintItemUpdateSnake,
    {
        id: String,
        seq: Option<i32>,
        file_path: Option<String>,
        title: Option<String>,
        audience: Option<String>,
        goal: Option<String>,
        must_cover: Option<String>,
        constraints: Option<String>
    },
    "blueprint_item_update",
    "Patch a blueprint_item; only provided fields change.",
    {
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "seq": { "type": "integer" },
            "file_path": { "type": "string" },
            "title": { "type": "string" },
            "audience": { "type": "string" },
            "goal": { "type": "string" },
            "must_cover": { "type": "string" },
            "constraints": { "type": "string" }
        },
        "required": ["id"]
    },
    exec_blueprint_item_update
);

project_tool!(
    BlueprintItemRemoveTool,
    BlueprintItemRemoveParams,
    { id: String },
    "blueprint_item_remove",
    "Delete a blueprint item by id.",
    {
        "type": "object",
        "properties": { "id": { "type": "string" } },
        "required": ["id"]
    },
    exec_blueprint_item_remove
);

project_tool!(
    TaskCreateTool,
    TaskCreateParams,
    {
        title: String,
        goal: String,
        parent_id: Option<String>,
        blueprint_item_id: Option<String>,
        acceptance: Option<String>,
        priority: Option<i32>
    },
    "task_create",
    "Create a task (default status backlog). Optional parent_id for subtree; link blueprint_item_id when tied to a doc requirement.",
    {
        "type": "object",
        "properties": {
            "title": { "type": "string" },
            "goal": { "type": "string" },
            "parent_id": { "type": "string" },
            "blueprint_item_id": { "type": "string" },
            "acceptance": { "type": "string" },
            "priority": { "type": "integer" }
        },
        "required": ["title", "goal"]
    },
    exec_task_create
);

project_tool!(
    TaskListTool,
    TaskListParams,
    {
        status: Option<String>,
        parent_id: Option<String>,
        blueprint_item_id: Option<String>
    },
    "task_list",
    "List tasks with optional filters: status, parent_id, blueprint_item_id.",
    {
        "type": "object",
        "properties": {
            "status": { "type": "string" },
            "parent_id": { "type": "string" },
            "blueprint_item_id": { "type": "string" }
        }
    },
    exec_task_list
);

project_tool!(
    TaskGetTool,
    TaskGetParams,
    { id: String },
    "task_get",
    "Get task detail including steps, runs, checkpoints, locks, artifacts.",
    {
        "type": "object",
        "properties": { "id": { "type": "string" } },
        "required": ["id"]
    },
    exec_task_get
);

project_tool!(
    TaskGetTreeTool,
    TaskGetTreeParams,
    { root_id: String },
    "task_get_tree",
    "Return task subtree from root_id with per-node summaries.",
    {
        "type": "object",
        "properties": { "root_id": { "type": "string" } },
        "required": ["root_id"]
    },
    exec_task_get_tree
);

project_tool!(
    TaskUpdateTool,
    TaskUpdateSnake,
    {
        id: String,
        title: Option<String>,
        goal: Option<String>,
        acceptance: Option<String>,
        status: Option<String>,
        priority: Option<i32>,
        blocked_reason: Option<String>,
        tags: Option<String>,
        conversation_ref: Option<String>
    },
    "task_update",
    "Patch task fields (status, priority, goals, tags, conversation_ref, etc.).",
    {
        "type": "object",
        "properties": {
            "id": { "type": "string" },
            "title": { "type": "string" },
            "goal": { "type": "string" },
            "acceptance": { "type": "string" },
            "status": { "type": "string" },
            "priority": { "type": "integer" },
            "blocked_reason": { "type": "string" },
            "tags": { "type": "string" },
            "conversation_ref": { "type": "string" }
        },
        "required": ["id"]
    },
    exec_task_update
);

project_tool!(
    TaskDeleteTool,
    TaskDeleteParams,
    { id: String },
    "task_delete",
    "Delete a task by id (cascade per schema).",
    {
        "type": "object",
        "properties": { "id": { "type": "string" } },
        "required": ["id"]
    },
    exec_task_delete
);

project_tool!(
    TaskStartRunTool,
    TaskStartRunParams,
    { task_id: String },
    "task_start_run",
    "Start a new TaskRun in running status for the given task.",
    {
        "type": "object",
        "properties": { "task_id": { "type": "string" } },
        "required": ["task_id"]
    },
    exec_task_start_run
);

project_tool!(
    TaskEndRunTool,
    TaskEndRunParams,
    {
        run_id: String,
        status: String,
        summary: Option<String>,
        error: Option<String>
    },
    "task_end_run",
    "Finish a TaskRun: set status and optional summary/error; sets ended_at.",
    {
        "type": "object",
        "properties": {
            "run_id": { "type": "string" },
            "status": { "type": "string", "description": "e.g. completed | failed" },
            "summary": { "type": "string" },
            "error": { "type": "string" }
        },
        "required": ["run_id", "status"]
    },
    exec_task_end_run
);

project_tool!(
    TaskAppendStepTool,
    TaskAppendStepParams,
    { task_id: String, title: String },
    "task_append_step",
    "Append a linear step (seq auto) with status pending.",
    {
        "type": "object",
        "properties": {
            "task_id": { "type": "string" },
            "title": { "type": "string" }
        },
        "required": ["task_id", "title"]
    },
    exec_task_append_step
);

project_tool!(
    TaskUpdateStepTool,
    TaskUpdateStepParams,
    {
        step_id: String,
        status: Option<String>,
        title: Option<String>
    },
    "task_update_step",
    "Patch task_step status and/or title.",
    {
        "type": "object",
        "properties": {
            "step_id": { "type": "string" },
            "status": { "type": "string" },
            "title": { "type": "string" }
        },
        "required": ["step_id"]
    },
    exec_task_update_step
);

project_tool!(
    TaskOpenCheckpointTool,
    TaskOpenCheckpointParams,
    { task_id: String, conversation_ref: String },
    "task_open_checkpoint",
    "Open a checkpoint and set task status waiting_checkpoint.",
    {
        "type": "object",
        "properties": {
            "task_id": { "type": "string" },
            "conversation_ref": { "type": "string", "description": "Message id or ref for resume" }
        },
        "required": ["task_id", "conversation_ref"]
    },
    exec_task_open_checkpoint
);

project_tool!(
    TaskCloseCheckpointTool,
    TaskCloseCheckpointParams,
    { checkpoint_id: String },
    "task_close_checkpoint",
    "Close checkpoint; may restore task from waiting_checkpoint to running.",
    {
        "type": "object",
        "properties": { "checkpoint_id": { "type": "string" } },
        "required": ["checkpoint_id"]
    },
    exec_task_close_checkpoint
);

project_tool!(
    TaskAcquireLockTool,
    TaskAcquireLockParams,
    {
        task_id: String,
        path: String,
        expires_at: Option<String>
    },
    "task_acquire_lock",
    "Acquire exclusive path lock for a task (SQLite uniqueness). expires_at optional ISO string.",
    {
        "type": "object",
        "properties": {
            "task_id": { "type": "string" },
            "path": { "type": "string", "description": "Workspace-relative or normalized path" },
            "expires_at": { "type": "string" }
        },
        "required": ["task_id", "path"]
    },
    exec_task_acquire_lock
);

project_tool!(
    TaskReleaseLockTool,
    TaskReleaseLockParams,
    { lock_id: String },
    "task_release_lock",
    "Release a path lock by id.",
    {
        "type": "object",
        "properties": { "lock_id": { "type": "string" } },
        "required": ["lock_id"]
    },
    exec_task_release_lock
);

project_tool!(
    TaskAddArtifactTool,
    TaskAddArtifactParams,
    {
        task_id: String,
        kind: String,
        path: String,
        content: Option<String>,
        run_id: Option<String>
    },
    "task_add_artifact",
    "Record an artifact (kind: file, summary, report, reference, ...).",
    {
        "type": "object",
        "properties": {
            "task_id": { "type": "string" },
            "kind": { "type": "string" },
            "path": { "type": "string" },
            "content": { "type": "string" },
            "run_id": { "type": "string" }
        },
        "required": ["task_id", "kind", "path"]
    },
    exec_task_add_artifact
);

// --- Exec fns (must follow macro-generated param types) ---

async fn exec_blueprint_create(db: &DatabaseConnection, p: BlueprintCreateParams) -> ToolResult {
    let r = ops::blueprint_create(db, p.title, p.goal, p.audience, p.style_guide)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_blueprint_get(db: &DatabaseConnection, p: BlueprintGetParams) -> ToolResult {
    let r = ops::blueprint_get(db, p.id).await.map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_blueprint_list(db: &DatabaseConnection, p: BlueprintListParams) -> ToolResult {
    let r = ops::blueprint_list(db, p.status)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_blueprint_update(db: &DatabaseConnection, p: BlueprintUpdateParams) -> ToolResult {
    let r = ops::blueprint_update(db, p.id, p.title, p.goal, p.audience, p.style_guide)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_blueprint_set_status(
    db: &DatabaseConnection,
    p: BlueprintSetStatusParams,
) -> ToolResult {
    let r = ops::blueprint_set_status(db, p.id, p.action)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_blueprint_item_add(db: &DatabaseConnection, p: BlueprintItemAddSnake) -> ToolResult {
    let r = ops::blueprint_item_add(
        db,
        BlueprintItemAddParams {
            blueprint_id: p.blueprint_id,
            file_path: p.file_path,
            title: p.title,
            audience: p.audience,
            goal: p.goal,
            must_cover: p.must_cover,
            constraints: p.constraints,
        },
    )
    .await
    .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_blueprint_item_update(
    db: &DatabaseConnection,
    p: BlueprintItemUpdateSnake,
) -> ToolResult {
    let r = ops::blueprint_item_update(
        db,
        BlueprintItemUpdateParams {
            id: p.id,
            seq: p.seq,
            file_path: p.file_path,
            title: p.title,
            audience: p.audience,
            goal: p.goal,
            must_cover: p.must_cover,
            constraints: p.constraints,
        },
    )
    .await
    .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_blueprint_item_remove(
    db: &DatabaseConnection,
    p: BlueprintItemRemoveParams,
) -> ToolResult {
    ops::blueprint_item_remove(db, p.id)
        .await
        .map_err(db_tool_err)?;
    Ok(json!({ "ok": true }))
}

async fn exec_task_create(db: &DatabaseConnection, p: TaskCreateParams) -> ToolResult {
    let r = ops::task_create(
        db,
        p.title,
        p.goal,
        p.parent_id,
        p.blueprint_item_id,
        p.acceptance,
        p.priority,
    )
    .await
    .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_list(db: &DatabaseConnection, p: TaskListParams) -> ToolResult {
    let r = ops::task_list(db, p.status, p.parent_id, p.blueprint_item_id)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_get(db: &DatabaseConnection, p: TaskGetParams) -> ToolResult {
    let r = ops::task_get(db, p.id).await.map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_get_tree(db: &DatabaseConnection, p: TaskGetTreeParams) -> ToolResult {
    let r = ops::task_get_tree(db, p.root_id)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_update(db: &DatabaseConnection, p: TaskUpdateSnake) -> ToolResult {
    let r = ops::task_update(
        db,
        TaskUpdateParams {
            id: p.id,
            title: p.title,
            goal: p.goal,
            acceptance: p.acceptance,
            status: p.status,
            priority: p.priority,
            blocked_reason: p.blocked_reason,
            tags: p.tags,
            conversation_ref: p.conversation_ref,
        },
    )
    .await
    .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_delete(db: &DatabaseConnection, p: TaskDeleteParams) -> ToolResult {
    ops::task_delete(db, p.id).await.map_err(db_tool_err)?;
    Ok(json!({ "ok": true }))
}

async fn exec_task_start_run(db: &DatabaseConnection, p: TaskStartRunParams) -> ToolResult {
    let r = ops::task_start_run(db, p.task_id)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_end_run(db: &DatabaseConnection, p: TaskEndRunParams) -> ToolResult {
    let r = ops::task_end_run(db, p.run_id, p.status, p.summary, p.error)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_append_step(db: &DatabaseConnection, p: TaskAppendStepParams) -> ToolResult {
    let r = ops::task_append_step(db, p.task_id, p.title)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_update_step(db: &DatabaseConnection, p: TaskUpdateStepParams) -> ToolResult {
    let r = ops::task_update_step(db, p.step_id, p.status, p.title)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_open_checkpoint(
    db: &DatabaseConnection,
    p: TaskOpenCheckpointParams,
) -> ToolResult {
    let r = ops::task_open_checkpoint(db, p.task_id, p.conversation_ref)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_close_checkpoint(
    db: &DatabaseConnection,
    p: TaskCloseCheckpointParams,
) -> ToolResult {
    let r = ops::task_close_checkpoint(db, p.checkpoint_id)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_acquire_lock(db: &DatabaseConnection, p: TaskAcquireLockParams) -> ToolResult {
    let r = ops::task_acquire_lock(db, p.task_id, p.path, p.expires_at)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

async fn exec_task_release_lock(db: &DatabaseConnection, p: TaskReleaseLockParams) -> ToolResult {
    ops::task_release_lock(db, p.lock_id)
        .await
        .map_err(db_tool_err)?;
    Ok(json!({ "ok": true }))
}

async fn exec_task_add_artifact(db: &DatabaseConnection, p: TaskAddArtifactParams) -> ToolResult {
    let r = ops::task_add_artifact(db, p.task_id, p.kind, p.path, p.content, p.run_id)
        .await
        .map_err(db_tool_err)?;
    to_json(r)
}

// --- Preview (no workspace) ---

pub(crate) struct PreviewRenderTool;

#[async_trait::async_trait]
impl Tool for PreviewRenderTool {
    fn name(&self) -> &str {
        "preview_render"
    }

    fn description(&self) -> &str {
        "Render Markdown to HTML preview (stub Comrak); returns diagnostics array and theme revision."
    }

    fn schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "content": { "type": "string", "description": "Markdown source" },
                "snapshot_id": { "type": "string", "description": "Optional id; default editor-buffer" }
            },
            "required": ["content"]
        })
    }

    async fn execute(&self, params: Value) -> ToolResult {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        struct P {
            content: String,
            snapshot_id: Option<String>,
        }
        let p: P = serde_json::from_value(params).map_err(invalid_params)?;
        let id = p.snapshot_id.unwrap_or_else(|| "editor-buffer".into());
        let r = ComrakStubBackend::render(&p.content, id);
        to_json(r)
    }
}

/// 规划对话智能体：蓝图全量 + 任务读/写（与设计文档权限表一致）。
pub fn planner_project_tools(state: SharedProject) -> Vec<Arc<dyn Tool>> {
    vec![
        Arc::new(BlueprintCreateTool::new(state.clone())),
        Arc::new(BlueprintGetTool::new(state.clone())),
        Arc::new(BlueprintListTool::new(state.clone())),
        Arc::new(BlueprintUpdateTool::new(state.clone())),
        Arc::new(BlueprintSetStatusTool::new(state.clone())),
        Arc::new(BlueprintItemAddTool::new(state.clone())),
        Arc::new(BlueprintItemUpdateTool::new(state.clone())),
        Arc::new(BlueprintItemRemoveTool::new(state.clone())),
        Arc::new(TaskCreateTool::new(state.clone())),
        Arc::new(TaskListTool::new(state.clone())),
        Arc::new(TaskGetTool::new(state.clone())),
        Arc::new(TaskGetTreeTool::new(state.clone())),
        Arc::new(TaskUpdateTool::new(state.clone())),
        Arc::new(TaskDeleteTool::new(state)),
    ]
}

/// 文档执行智能体：任务读/写、运行/步骤/检查点/锁/产出物（无蓝图工具）。
pub fn executor_project_tools(state: SharedProject) -> Vec<Arc<dyn Tool>> {
    vec![
        Arc::new(TaskGetTool::new(state.clone())),
        Arc::new(TaskGetTreeTool::new(state.clone())),
        Arc::new(TaskUpdateTool::new(state.clone())),
        Arc::new(TaskStartRunTool::new(state.clone())),
        Arc::new(TaskEndRunTool::new(state.clone())),
        Arc::new(TaskAppendStepTool::new(state.clone())),
        Arc::new(TaskUpdateStepTool::new(state.clone())),
        Arc::new(TaskOpenCheckpointTool::new(state.clone())),
        Arc::new(TaskCloseCheckpointTool::new(state.clone())),
        Arc::new(TaskAcquireLockTool::new(state.clone())),
        Arc::new(TaskReleaseLockTool::new(state.clone())),
        Arc::new(TaskAddArtifactTool::new(state)),
    ]
}

pub fn preview_render_tool() -> Arc<dyn Tool> {
    Arc::new(PreviewRenderTool)
}

/// 将已注册的 agent 工具转为 lmkit 请求体中的 `tools` 列表。
pub fn definitions_for_lmkit(tools: &[Arc<dyn Tool>]) -> Vec<ToolDefinition> {
    tools
        .iter()
        .map(|t| ToolDefinition::function_with_description(t.name(), t.description(), t.schema()))
        .collect()
}
