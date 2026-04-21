use std::ops::Deref;

use sea_orm::DbErr;
use tauri::{AppHandle, State};

use crate::app::checkpoint::CheckpointBridge;
use crate::app::project::{
    self,
    dto::{
        BlueprintDetailDto, BlueprintDto, BlueprintItemDto, OutputDto, PathLockDto, ReviewDto,
        TaskDetailDto, TaskDto, TaskRunDto, TaskStepDto, TaskTreeNodeDto,
    },
    BlueprintItemAddParams, BlueprintItemUpdateParams, TaskUpdateParams,
};
use crate::app::state::{SharedActiveContext, SharedProject};

async fn db(
    state: &State<'_, SharedProject>,
) -> Result<std::sync::Arc<crate::app::project::ProjectContext>, String> {
    state
        .0
        .lock()
        .await
        .focused_context()
        .ok_or_else(|| "workspace not opened".to_owned())
}

fn de(e: DbErr) -> String {
    e.to_string()
}

// --- Blueprint ---

#[tauri::command]
pub async fn blueprint_create(
    state: State<'_, SharedProject>,
    title: String,
    goal: Option<String>,
    audience: Option<String>,
    style_guide: Option<String>,
) -> Result<BlueprintDto, String> {
    let ctx = db(&state).await?;
    project::blueprint_create(&ctx.db, title, goal, audience, style_guide)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn blueprint_get(
    state: State<'_, SharedProject>,
    id: String,
) -> Result<BlueprintDetailDto, String> {
    let ctx = db(&state).await?;
    project::blueprint_get(&ctx.db, id).await.map_err(de)
}

#[tauri::command]
pub async fn blueprint_list(
    state: State<'_, SharedProject>,
    status: Option<String>,
) -> Result<Vec<BlueprintDto>, String> {
    let ctx = db(&state).await?;
    project::blueprint_list(&ctx.db, status).await.map_err(de)
}

#[tauri::command]
pub async fn blueprint_update(
    state: State<'_, SharedProject>,
    id: String,
    title: Option<String>,
    goal: Option<String>,
    audience: Option<String>,
    style_guide: Option<String>,
) -> Result<BlueprintDto, String> {
    let ctx = db(&state).await?;
    project::blueprint_update(&ctx.db, id, title, goal, audience, style_guide)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn blueprint_set_status(
    state: State<'_, SharedProject>,
    id: String,
    action: String,
) -> Result<BlueprintDto, String> {
    let ctx = db(&state).await?;
    project::blueprint_set_status(&ctx.db, id, action)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn blueprint_item_add(
    state: State<'_, SharedProject>,
    p: BlueprintItemAddParams,
) -> Result<BlueprintItemDto, String> {
    let ctx = db(&state).await?;
    project::blueprint_item_add(&ctx.db, p).await.map_err(de)
}

#[tauri::command]
pub async fn blueprint_item_update(
    state: State<'_, SharedProject>,
    p: BlueprintItemUpdateParams,
) -> Result<BlueprintItemDto, String> {
    let ctx = db(&state).await?;
    project::blueprint_item_update(&ctx.db, p).await.map_err(de)
}

#[tauri::command]
pub async fn blueprint_item_remove(
    state: State<'_, SharedProject>,
    id: String,
) -> Result<(), String> {
    let ctx = db(&state).await?;
    project::blueprint_item_remove(&ctx.db, id)
        .await
        .map_err(de)
}

// --- Task ---

#[tauri::command]
pub async fn task_create(
    state: State<'_, SharedProject>,
    title: String,
    goal: String,
    parent_id: Option<String>,
    blueprint_item_id: Option<String>,
    acceptance: Option<String>,
    priority: Option<i32>,
) -> Result<TaskDto, String> {
    let ctx = db(&state).await?;
    project::task_create(
        &ctx.db,
        title,
        goal,
        parent_id,
        blueprint_item_id,
        acceptance,
        priority,
    )
    .await
    .map_err(de)
}

#[tauri::command]
pub async fn task_list(
    state: State<'_, SharedProject>,
    status: Option<String>,
    parent_id: Option<String>,
    blueprint_item_id: Option<String>,
) -> Result<Vec<TaskDto>, String> {
    let ctx = db(&state).await?;
    project::task_list(&ctx.db, status, parent_id, blueprint_item_id)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn task_get(
    state: State<'_, SharedProject>,
    id: String,
) -> Result<TaskDetailDto, String> {
    let ctx = db(&state).await?;
    project::task_get(&ctx.db, id).await.map_err(de)
}

#[tauri::command]
pub async fn task_get_tree(
    state: State<'_, SharedProject>,
    root_id: String,
) -> Result<TaskTreeNodeDto, String> {
    let ctx = db(&state).await?;
    project::task_get_tree(&ctx.db, root_id).await.map_err(de)
}

#[tauri::command]
pub async fn task_update(
    state: State<'_, SharedProject>,
    p: TaskUpdateParams,
) -> Result<TaskDto, String> {
    let ctx = db(&state).await?;
    project::task_update(&ctx.db, p).await.map_err(de)
}

#[tauri::command]
pub async fn task_delete(state: State<'_, SharedProject>, id: String) -> Result<(), String> {
    let ctx = db(&state).await?;
    project::task_delete(&ctx.db, id).await.map_err(de)
}

#[tauri::command]
pub async fn task_start_run(
    state: State<'_, SharedProject>,
    task_id: String,
) -> Result<TaskRunDto, String> {
    let ctx = db(&state).await?;
    project::task_start_run(&ctx.db, task_id).await.map_err(de)
}

#[tauri::command]
pub async fn task_end_run(
    state: State<'_, SharedProject>,
    run_id: String,
    status: String,
    summary: Option<String>,
    error: Option<String>,
) -> Result<TaskRunDto, String> {
    let ctx = db(&state).await?;
    project::task_end_run(&ctx.db, run_id, status, summary, error)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn task_append_step(
    state: State<'_, SharedProject>,
    task_id: String,
    title: String,
) -> Result<TaskStepDto, String> {
    let ctx = db(&state).await?;
    project::task_append_step(&ctx.db, task_id, title)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn task_update_step(
    state: State<'_, SharedProject>,
    step_id: String,
    status: Option<String>,
    title: Option<String>,
) -> Result<TaskStepDto, String> {
    let ctx = db(&state).await?;
    project::task_update_step(&ctx.db, step_id, status, title)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn task_open_checkpoint(
    app: AppHandle,
    active: State<'_, SharedActiveContext>,
    state: State<'_, SharedProject>,
    task_id: String,
    conversation_ref: String,
) -> Result<ReviewDto, String> {
    let ctx = db(&state).await?;
    let cp = project::task_open_review(&ctx.db, task_id, conversation_ref)
        .await
        .map_err(de)?;
    let bridge = CheckpointBridge::new(app.clone(), active.deref().clone());
    bridge.notify_opened(&cp).await;
    Ok(cp)
}

#[tauri::command]
pub async fn task_close_checkpoint(
    app: AppHandle,
    active: State<'_, SharedActiveContext>,
    state: State<'_, SharedProject>,
    checkpoint_id: String,
) -> Result<ReviewDto, String> {
    let ctx = db(&state).await?;
    let cp = project::task_close_review(&ctx.db, checkpoint_id)
        .await
        .map_err(de)?;
    let task_status_after = project::task_get(&ctx.db, cp.task_id.clone())
        .await
        .ok()
        .map(|d| d.task.status);
    let bridge = CheckpointBridge::new(app.clone(), active.deref().clone());
    bridge.notify_closed(&cp, task_status_after).await;
    Ok(cp)
}

#[tauri::command]
pub async fn task_acquire_lock(
    state: State<'_, SharedProject>,
    task_id: String,
    path: String,
    expires_at: Option<String>,
) -> Result<PathLockDto, String> {
    let ctx = db(&state).await?;
    project::task_acquire_lock(&ctx.db, task_id, path, expires_at)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn task_release_lock(
    state: State<'_, SharedProject>,
    lock_id: String,
) -> Result<(), String> {
    let ctx = db(&state).await?;
    project::task_release_lock(&ctx.db, lock_id)
        .await
        .map_err(de)
}

#[tauri::command]
pub async fn task_add_artifact(
    state: State<'_, SharedProject>,
    task_id: String,
    kind: String,
    path: String,
    content: Option<String>,
    run_id: Option<String>,
) -> Result<OutputDto, String> {
    let ctx = db(&state).await?;
    project::task_add_output(&ctx.db, task_id, kind, path, content, run_id)
        .await
        .map_err(de)
}
