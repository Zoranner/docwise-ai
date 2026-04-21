use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    Set,
};
use uuid::Uuid;

use super::dto::{
    BlueprintDetailDto, BlueprintDto, BlueprintItemDto, OutputDto, PathLockDto, ReviewDto,
    TaskDetailDto, TaskDto, TaskRunDto, TaskStepDto, TaskSubtreeSummaryDto, TaskTreeNodeDto,
};
use super::entity::{
    blueprint, blueprint_item, output, path_lock, review, task, task_run, task_step,
};
use super::params::{BlueprintItemAddParams, BlueprintItemUpdateParams, TaskUpdateParams};
use super::util::now_iso;

fn new_id() -> String {
    Uuid::new_v4().to_string()
}

// --- Blueprint ---

pub async fn blueprint_create(
    db: &DatabaseConnection,
    title: String,
    goal: Option<String>,
    audience: Option<String>,
    style_guide: Option<String>,
) -> Result<BlueprintDto, DbErr> {
    let ts = now_iso();
    let id = new_id();
    let m = blueprint::ActiveModel {
        id: Set(id.clone()),
        title: Set(title),
        status: Set("draft".to_owned()),
        goal: Set(goal.unwrap_or_default()),
        audience: Set(audience.unwrap_or_default()),
        style_guide: Set(style_guide.unwrap_or_else(|| "{}".to_owned())),
        created_at: Set(ts.clone()),
        updated_at: Set(ts),
    };
    blueprint::Entity::insert(m).exec(db).await?;
    blueprint::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("blueprint insert failed".to_owned()))
}

pub async fn blueprint_get(
    db: &DatabaseConnection,
    id: String,
) -> Result<BlueprintDetailDto, DbErr> {
    let bp = blueprint::Entity::find_by_id(id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("blueprint not found".to_owned()))?;
    let items = blueprint_item::Entity::find()
        .filter(blueprint_item::Column::BlueprintId.eq(id))
        .order_by_asc(blueprint_item::Column::Seq)
        .all(db)
        .await?;
    Ok(BlueprintDetailDto {
        blueprint: bp.into(),
        items: items.into_iter().map(Into::into).collect(),
    })
}

pub async fn blueprint_list(
    db: &DatabaseConnection,
    status: Option<String>,
) -> Result<Vec<BlueprintDto>, DbErr> {
    let mut q = blueprint::Entity::find();
    if let Some(s) = status {
        q = q.filter(blueprint::Column::Status.eq(s));
    }
    let rows = q
        .order_by_desc(blueprint::Column::UpdatedAt)
        .all(db)
        .await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn blueprint_update(
    db: &DatabaseConnection,
    id: String,
    title: Option<String>,
    goal: Option<String>,
    audience: Option<String>,
    style_guide: Option<String>,
) -> Result<BlueprintDto, DbErr> {
    let m = blueprint::Entity::find_by_id(id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("blueprint not found".to_owned()))?;
    let mut a: blueprint::ActiveModel = m.into();
    if let Some(v) = title {
        a.title = Set(v);
    }
    if let Some(v) = goal {
        a.goal = Set(v);
    }
    if let Some(v) = audience {
        a.audience = Set(v);
    }
    if let Some(v) = style_guide {
        a.style_guide = Set(v);
    }
    a.updated_at = Set(now_iso());
    a.update(db).await?;
    blueprint::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("blueprint not found after update".to_owned()))
}

/// `action`: `approve` | `archive` | `supersede` | `activate`
pub async fn blueprint_set_status(
    db: &DatabaseConnection,
    id: String,
    action: String,
) -> Result<BlueprintDto, DbErr> {
    let m = blueprint::Entity::find_by_id(id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("blueprint not found".to_owned()))?;
    let mut a: blueprint::ActiveModel = m.clone().into();
    let next = match (m.status.as_str(), action.as_str()) {
        (_, "archive") => "archived",
        (_, "supersede") => "superseded",
        ("draft" | "revised", "approve") => "approved",
        ("approved", "activate") => "active",
        _ => {
            return Err(DbErr::Custom(format!(
                "invalid blueprint transition: status={} action={}",
                m.status, action
            )));
        }
    };
    a.status = Set(next.to_owned());
    a.updated_at = Set(now_iso());
    a.update(db).await?;
    blueprint::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("blueprint not found".to_owned()))
}

pub async fn blueprint_item_add(
    db: &DatabaseConnection,
    p: BlueprintItemAddParams,
) -> Result<BlueprintItemDto, DbErr> {
    let BlueprintItemAddParams {
        blueprint_id,
        file_path,
        title,
        audience,
        goal,
        must_cover,
        constraints,
    } = p;
    let _ = blueprint::Entity::find_by_id(blueprint_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("blueprint not found".to_owned()))?;
    let max_seq: Option<i32> = blueprint_item::Entity::find()
        .filter(blueprint_item::Column::BlueprintId.eq(blueprint_id.clone()))
        .order_by_desc(blueprint_item::Column::Seq)
        .one(db)
        .await?
        .map(|r| r.seq);
    let seq = max_seq.map(|s| s + 1).unwrap_or(0);
    let ts = now_iso();
    let id = new_id();
    let row = blueprint_item::ActiveModel {
        id: Set(id.clone()),
        blueprint_id: Set(blueprint_id),
        seq: Set(seq),
        file_path: Set(file_path),
        title: Set(title),
        audience: Set(audience.unwrap_or_default()),
        goal: Set(goal.unwrap_or_default()),
        must_cover: Set(must_cover.unwrap_or_else(|| "[]".to_owned())),
        constraints: Set(constraints.unwrap_or_else(|| "[]".to_owned())),
        created_at: Set(ts.clone()),
        updated_at: Set(ts),
    };
    blueprint_item::Entity::insert(row).exec(db).await?;
    blueprint_item::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("blueprint_item insert failed".to_owned()))
}

pub async fn blueprint_item_update(
    db: &DatabaseConnection,
    p: BlueprintItemUpdateParams,
) -> Result<BlueprintItemDto, DbErr> {
    let BlueprintItemUpdateParams {
        id,
        seq,
        file_path,
        title,
        audience,
        goal,
        must_cover,
        constraints,
    } = p;
    let m = blueprint_item::Entity::find_by_id(id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("blueprint_item not found".to_owned()))?;
    let mut a: blueprint_item::ActiveModel = m.into();
    if let Some(v) = seq {
        a.seq = Set(v);
    }
    if let Some(v) = file_path {
        a.file_path = Set(v);
    }
    if let Some(v) = title {
        a.title = Set(v);
    }
    if let Some(v) = audience {
        a.audience = Set(v);
    }
    if let Some(v) = goal {
        a.goal = Set(v);
    }
    if let Some(v) = must_cover {
        a.must_cover = Set(v);
    }
    if let Some(v) = constraints {
        a.constraints = Set(v);
    }
    a.updated_at = Set(now_iso());
    a.update(db).await?;
    blueprint_item::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("blueprint_item not found".to_owned()))
}

pub async fn blueprint_item_remove(db: &DatabaseConnection, id: String) -> Result<(), DbErr> {
    blueprint_item::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

// --- Task ---

pub async fn task_create(
    db: &DatabaseConnection,
    title: String,
    goal: String,
    parent_id: Option<String>,
    blueprint_item_id: Option<String>,
    acceptance: Option<String>,
    priority: Option<i32>,
) -> Result<TaskDto, DbErr> {
    if let Some(ref pid) = parent_id {
        let _ = task::Entity::find_by_id(pid.clone())
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("parent task not found".to_owned()))?;
    }
    if let Some(ref bid) = blueprint_item_id {
        let _ = blueprint_item::Entity::find_by_id(bid.clone())
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("blueprint_item not found".to_owned()))?;
    }
    let ts = now_iso();
    let id = new_id();
    let row = task::ActiveModel {
        id: Set(id.clone()),
        blueprint_item_id: Set(blueprint_item_id),
        parent_id: Set(parent_id),
        conversation_ref: Set(String::new()),
        title: Set(title),
        goal: Set(goal),
        acceptance: Set(acceptance.unwrap_or_default()),
        status: Set("backlog".to_owned()),
        priority: Set(priority.unwrap_or(0)),
        blocked_reason: Set(String::new()),
        tags: Set("[]".to_owned()),
        created_at: Set(ts.clone()),
        updated_at: Set(ts),
    };
    task::Entity::insert(row).exec(db).await?;
    task::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("task insert failed".to_owned()))
}

pub async fn task_list(
    db: &DatabaseConnection,
    status: Option<String>,
    parent_id: Option<String>,
    blueprint_item_id: Option<String>,
) -> Result<Vec<TaskDto>, DbErr> {
    let mut q = task::Entity::find();
    if let Some(s) = status {
        q = q.filter(task::Column::Status.eq(s));
    }
    if let Some(p) = parent_id {
        q = q.filter(task::Column::ParentId.eq(p));
    }
    if let Some(b) = blueprint_item_id {
        q = q.filter(task::Column::BlueprintItemId.eq(b));
    }
    let rows = q.order_by_asc(task::Column::Priority).all(db).await?;
    Ok(rows.into_iter().map(Into::into).collect())
}

pub async fn task_get(db: &DatabaseConnection, id: String) -> Result<TaskDetailDto, DbErr> {
    let t = task::Entity::find_by_id(id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    let steps = task_step::Entity::find()
        .filter(task_step::Column::TaskId.eq(id.clone()))
        .order_by_asc(task_step::Column::Seq)
        .all(db)
        .await?;
    let runs = task_run::Entity::find()
        .filter(task_run::Column::TaskId.eq(id.clone()))
        .order_by_desc(task_run::Column::StartedAt)
        .all(db)
        .await?;
    let reviews = review::Entity::find()
        .filter(review::Column::TaskId.eq(id.clone()))
        .order_by_desc(review::Column::CreatedAt)
        .all(db)
        .await?;
    let locks = path_lock::Entity::find()
        .filter(path_lock::Column::TaskId.eq(id.clone()))
        .all(db)
        .await?;
    let outputs = output::Entity::find()
        .filter(output::Column::TaskId.eq(id.clone()))
        .order_by_desc(output::Column::CreatedAt)
        .all(db)
        .await?;
    Ok(TaskDetailDto {
        task: t.into(),
        steps: steps.into_iter().map(Into::into).collect(),
        runs: runs.into_iter().map(Into::into).collect(),
        reviews: reviews.into_iter().map(Into::into).collect(),
        path_locks: locks.into_iter().map(Into::into).collect(),
        outputs: outputs.into_iter().map(Into::into).collect(),
    })
}

fn summarize_tasks(nodes: &[task::Model]) -> TaskSubtreeSummaryDto {
    let mut by_status: BTreeMap<String, u32> = BTreeMap::new();
    for n in nodes {
        *by_status.entry(n.status.clone()).or_insert(0) += 1;
    }
    TaskSubtreeSummaryDto {
        total: nodes.len(),
        by_status,
    }
}

fn build_task_tree(root_id: &str, flat: &[task::Model]) -> TaskTreeNodeDto {
    let by_id: HashMap<String, task::Model> =
        flat.iter().cloned().map(|t| (t.id.clone(), t)).collect();
    let mut by_parent: HashMap<Option<String>, Vec<task::Model>> =
        flat.iter().cloned().fold(HashMap::new(), |mut acc, t| {
            acc.entry(t.parent_id.clone()).or_default().push(t);
            acc
        });
    for v in by_parent.values_mut() {
        v.sort_by(|a, b| {
            a.priority
                .cmp(&b.priority)
                .then_with(|| a.created_at.cmp(&b.created_at))
        });
    }
    fn build_recursive(
        id: &str,
        by_parent: &HashMap<Option<String>, Vec<task::Model>>,
        by_id: &HashMap<String, task::Model>,
    ) -> TaskTreeNodeDto {
        let model = by_id[id].clone();
        let raw_children = by_parent
            .get(&Some(id.to_string()))
            .cloned()
            .unwrap_or_default();
        let children: Vec<TaskTreeNodeDto> = raw_children
            .iter()
            .map(|c| build_recursive(&c.id, by_parent, by_id))
            .collect();
        TaskTreeNodeDto {
            task: model.into(),
            children,
            summary: TaskSubtreeSummaryDto {
                total: 0,
                by_status: BTreeMap::new(),
            },
        }
    }
    let mut tree = build_recursive(root_id, &by_parent, &by_id);
    tree.summary = summarize_tasks(flat);
    tree
}

pub async fn task_get_tree(
    db: &DatabaseConnection,
    root_id: String,
) -> Result<TaskTreeNodeDto, DbErr> {
    let _root = task::Entity::find_by_id(root_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    let mut collected: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(root_id.clone());
    collected.insert(root_id.clone());
    while let Some(pid) = queue.pop_front() {
        let kids = task::Entity::find()
            .filter(task::Column::ParentId.eq(pid))
            .all(db)
            .await?;
        for k in kids {
            if collected.insert(k.id.clone()) {
                queue.push_back(k.id);
            }
        }
    }
    let ids: Vec<String> = collected.into_iter().collect();
    let flat = task::Entity::find()
        .filter(task::Column::Id.is_in(ids))
        .all(db)
        .await?;
    Ok(build_task_tree(&root_id, &flat))
}

pub async fn task_update(db: &DatabaseConnection, p: TaskUpdateParams) -> Result<TaskDto, DbErr> {
    let TaskUpdateParams {
        id,
        title,
        goal,
        acceptance,
        status,
        priority,
        blocked_reason,
        tags,
        conversation_ref,
    } = p;
    let m = task::Entity::find_by_id(id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    let mut a: task::ActiveModel = m.into();
    if let Some(v) = title {
        a.title = Set(v);
    }
    if let Some(v) = goal {
        a.goal = Set(v);
    }
    if let Some(v) = acceptance {
        a.acceptance = Set(v);
    }
    if let Some(v) = status {
        a.status = Set(v);
    }
    if let Some(v) = priority {
        a.priority = Set(v);
    }
    if let Some(v) = blocked_reason {
        a.blocked_reason = Set(v);
    }
    if let Some(v) = tags {
        a.tags = Set(v);
    }
    if let Some(v) = conversation_ref {
        a.conversation_ref = Set(v);
    }
    a.updated_at = Set(now_iso());
    a.update(db).await?;
    task::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))
}

pub async fn task_delete(db: &DatabaseConnection, id: String) -> Result<(), DbErr> {
    task::Entity::delete_by_id(id).exec(db).await?;
    Ok(())
}

pub async fn task_start_run(db: &DatabaseConnection, task_id: String) -> Result<TaskRunDto, DbErr> {
    let _ = task::Entity::find_by_id(task_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    let id = new_id();
    let ts = now_iso();
    let row = task_run::ActiveModel {
        id: Set(id.clone()),
        task_id: Set(task_id),
        status: Set("running".to_owned()),
        summary: Set(String::new()),
        error: Set(String::new()),
        started_at: Set(ts),
        ended_at: Set(None),
    };
    task_run::Entity::insert(row).exec(db).await?;
    task_run::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("task_run insert failed".to_owned()))
}

pub async fn task_end_run(
    db: &DatabaseConnection,
    run_id: String,
    status: String,
    summary: Option<String>,
    error: Option<String>,
) -> Result<TaskRunDto, DbErr> {
    let m = task_run::Entity::find_by_id(run_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task_run not found".to_owned()))?;
    let mut a: task_run::ActiveModel = m.into();
    a.status = Set(status);
    if let Some(s) = summary {
        a.summary = Set(s);
    }
    if let Some(e) = error {
        a.error = Set(e);
    }
    a.ended_at = Set(Some(now_iso()));
    a.update(db).await?;
    task_run::Entity::find_by_id(run_id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("task_run not found".to_owned()))
}

pub async fn task_append_step(
    db: &DatabaseConnection,
    task_id: String,
    title: String,
) -> Result<TaskStepDto, DbErr> {
    let _ = task::Entity::find_by_id(task_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    let max_seq: Option<i32> = task_step::Entity::find()
        .filter(task_step::Column::TaskId.eq(task_id.clone()))
        .order_by_desc(task_step::Column::Seq)
        .one(db)
        .await?
        .map(|r| r.seq);
    let seq = max_seq.map(|s| s + 1).unwrap_or(0);
    let ts = now_iso();
    let id = new_id();
    let row = task_step::ActiveModel {
        id: Set(id.clone()),
        task_id: Set(task_id),
        seq: Set(seq),
        title: Set(title),
        status: Set("pending".to_owned()),
        created_at: Set(ts.clone()),
        updated_at: Set(ts),
    };
    task_step::Entity::insert(row).exec(db).await?;
    task_step::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("task_step insert failed".to_owned()))
}

pub async fn task_update_step(
    db: &DatabaseConnection,
    step_id: String,
    status: Option<String>,
    title: Option<String>,
) -> Result<TaskStepDto, DbErr> {
    let m = task_step::Entity::find_by_id(step_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task_step not found".to_owned()))?;
    let mut a: task_step::ActiveModel = m.into();
    if let Some(v) = status {
        a.status = Set(v);
    }
    if let Some(v) = title {
        a.title = Set(v);
    }
    a.updated_at = Set(now_iso());
    a.update(db).await?;
    task_step::Entity::find_by_id(step_id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("task_step not found".to_owned()))
}

pub async fn task_open_review(
    db: &DatabaseConnection,
    task_id: String,
    conversation_ref: String,
) -> Result<ReviewDto, DbErr> {
    let _ = task::Entity::find_by_id(task_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    let ts = now_iso();
    let id = new_id();
    let row = review::ActiveModel {
        id: Set(id.clone()),
        task_id: Set(task_id.clone()),
        status: Set("open".to_owned()),
        conversation_ref: Set(conversation_ref),
        created_at: Set(ts.clone()),
        updated_at: Set(ts.clone()),
    };
    review::Entity::insert(row).exec(db).await?;
    let mut t: task::ActiveModel = task::Entity::find_by_id(task_id)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?
        .into();
    t.status = Set("waiting_checkpoint".to_owned());
    t.updated_at = Set(now_iso());
    t.update(db).await?;
    review::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("review insert failed".to_owned()))
}

pub async fn task_close_review(
    db: &DatabaseConnection,
    review_id: String,
) -> Result<ReviewDto, DbErr> {
    let m = review::Entity::find_by_id(review_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("review not found".to_owned()))?;
    let task_id = m.task_id.clone();
    let mut a: review::ActiveModel = m.into();
    a.status = Set("closed".to_owned());
    a.updated_at = Set(now_iso());
    a.update(db).await?;
    let task_row = task::Entity::find_by_id(task_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    if task_row.status == "waiting_checkpoint" {
        let mut t: task::ActiveModel = task_row.into();
        t.status = Set("running".to_owned());
        t.updated_at = Set(now_iso());
        t.update(db).await?;
    }
    review::Entity::find_by_id(review_id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("review not found".to_owned()))
}

pub async fn task_acquire_lock(
    db: &DatabaseConnection,
    task_id: String,
    path: String,
    expires_at: Option<String>,
) -> Result<PathLockDto, DbErr> {
    let _ = task::Entity::find_by_id(task_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    let id = new_id();
    let ts = now_iso();
    let row = path_lock::ActiveModel {
        id: Set(id.clone()),
        task_id: Set(task_id),
        path: Set(path),
        expires_at: Set(expires_at),
        created_at: Set(ts),
    };
    match path_lock::Entity::insert(row).exec(db).await {
        Ok(_) => {}
        Err(e) => {
            return Err(DbErr::Custom(format!(
                "path lock conflict or db error: {e}"
            )));
        }
    }
    path_lock::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("path_lock insert failed".to_owned()))
}

pub async fn task_release_lock(db: &DatabaseConnection, lock_id: String) -> Result<(), DbErr> {
    path_lock::Entity::delete_by_id(lock_id).exec(db).await?;
    Ok(())
}

pub async fn task_add_output(
    db: &DatabaseConnection,
    task_id: String,
    kind: String,
    path: String,
    content: Option<String>,
    run_id: Option<String>,
) -> Result<OutputDto, DbErr> {
    let _ = task::Entity::find_by_id(task_id.clone())
        .one(db)
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("task not found".to_owned()))?;
    if let Some(ref rid) = run_id {
        let _ = task_run::Entity::find_by_id(rid.clone())
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("task_run not found".to_owned()))?;
    }
    let id = new_id();
    let ts = now_iso();
    let row = output::ActiveModel {
        id: Set(id.clone()),
        task_id: Set(task_id),
        run_id: Set(run_id),
        kind: Set(kind),
        path: Set(path),
        content: Set(content.unwrap_or_default()),
        created_at: Set(ts),
    };
    output::Entity::insert(row).exec(db).await?;
    output::Entity::find_by_id(id)
        .one(db)
        .await?
        .map(Into::into)
        .ok_or_else(|| DbErr::RecordNotFound("output insert failed".to_owned()))
}
