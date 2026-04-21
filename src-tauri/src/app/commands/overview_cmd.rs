use serde::Serialize;
use tauri::State;

use sea_orm::{sea_query::Expr, ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

use crate::app::project::entity::{blueprint, checkpoint, task};
use crate::app::state::SharedProject;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCountDto {
    pub status: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceOverviewDto {
    pub workspace_id: String,
    pub path: String,
    pub focused: bool,
    pub task_by_status: Vec<StatusCountDto>,
    pub blueprint_by_status: Vec<StatusCountDto>,
    pub open_checkpoints: u32,
}

async fn task_status_counts(
    db: &sea_orm::DatabaseConnection,
) -> Result<Vec<StatusCountDto>, sea_orm::DbErr> {
    let rows = task::Entity::find()
        .select_only()
        .column(task::Column::Status)
        .column_as(Expr::cust("COUNT(*)"), "count")
        .group_by(task::Column::Status)
        .into_tuple::<(String, i64)>()
        .all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|(status, count)| StatusCountDto {
            status,
            count: count as u32,
        })
        .collect())
}

async fn blueprint_status_counts(
    db: &sea_orm::DatabaseConnection,
) -> Result<Vec<StatusCountDto>, sea_orm::DbErr> {
    let rows = blueprint::Entity::find()
        .select_only()
        .column(blueprint::Column::Status)
        .column_as(Expr::cust("COUNT(*)"), "count")
        .group_by(blueprint::Column::Status)
        .into_tuple::<(String, i64)>()
        .all(db)
        .await?;
    Ok(rows
        .into_iter()
        .map(|(status, count)| StatusCountDto {
            status,
            count: count as u32,
        })
        .collect())
}

/// 对当前进程内**已打开**的各工作区 `project.db` 做只读聚合，供总览 UI。
#[tauri::command]
pub async fn session_workspaces_overview(
    state: State<'_, SharedProject>,
) -> Result<Vec<WorkspaceOverviewDto>, String> {
    let host = state.0.lock().await;
    let focused = host.focused_workspace_id.as_deref();
    let mut out: Vec<WorkspaceOverviewDto> = Vec::new();

    for (wid, ctx) in &host.open {
        let db = &ctx.db;
        let task_by_status = task_status_counts(db).await.map_err(|e| e.to_string())?;
        let blueprint_by_status =
            blueprint_status_counts(db).await.map_err(|e| e.to_string())?;
        let open_checkpoints = checkpoint::Entity::find()
            .filter(checkpoint::Column::Status.eq("open"))
            .count(db)
            .await
            .map_err(|e| e.to_string())? as u32;

        out.push(WorkspaceOverviewDto {
            workspace_id: wid.clone(),
            path: ctx.workspace_root().to_string_lossy().into_owned(),
            focused: focused == Some(wid.as_str()),
            task_by_status,
            blueprint_by_status,
            open_checkpoints,
        });
    }

    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}
