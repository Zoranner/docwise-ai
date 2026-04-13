use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for stmt in schema_statements() {
            db.execute_unprepared(stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        for stmt in drop_statements() {
            db.execute_unprepared(stmt).await?;
        }
        Ok(())
    }
}

fn schema_statements() -> &'static [&'static str] {
    &[
        r#"CREATE TABLE IF NOT EXISTS blueprints (
    id          TEXT PRIMARY KEY,
    title       TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'draft',
    goal        TEXT NOT NULL DEFAULT '',
    audience    TEXT NOT NULL DEFAULT '',
    style_guide TEXT NOT NULL DEFAULT '',
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);"#,
        r#"CREATE TABLE IF NOT EXISTS blueprint_items (
    id           TEXT PRIMARY KEY,
    blueprint_id TEXT NOT NULL REFERENCES blueprints(id) ON DELETE CASCADE,
    seq          INTEGER NOT NULL,
    file_path    TEXT NOT NULL,
    title        TEXT NOT NULL,
    audience     TEXT NOT NULL DEFAULT '',
    goal         TEXT NOT NULL DEFAULT '',
    must_cover   TEXT NOT NULL DEFAULT '[]',
    constraints  TEXT NOT NULL DEFAULT '[]',
    created_at   TEXT NOT NULL,
    updated_at   TEXT NOT NULL
);"#,
        r#"CREATE TABLE IF NOT EXISTS tasks (
    id                 TEXT PRIMARY KEY,
    blueprint_item_id  TEXT REFERENCES blueprint_items(id) ON DELETE SET NULL,
    parent_id          TEXT REFERENCES tasks(id) ON DELETE CASCADE,
    conversation_ref   TEXT NOT NULL DEFAULT '',
    title              TEXT NOT NULL DEFAULT '',
    goal               TEXT NOT NULL DEFAULT '',
    acceptance         TEXT NOT NULL DEFAULT '',
    status             TEXT NOT NULL DEFAULT 'backlog',
    priority           INTEGER NOT NULL DEFAULT 0,
    blocked_reason     TEXT NOT NULL DEFAULT '',
    tags               TEXT NOT NULL DEFAULT '[]',
    created_at         TEXT NOT NULL,
    updated_at         TEXT NOT NULL
);"#,
        r#"CREATE TABLE IF NOT EXISTS task_steps (
    id         TEXT PRIMARY KEY,
    task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    seq        INTEGER NOT NULL,
    title      TEXT NOT NULL,
    status     TEXT NOT NULL DEFAULT 'pending',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);"#,
        r#"CREATE TABLE IF NOT EXISTS task_runs (
    id         TEXT PRIMARY KEY,
    task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    status     TEXT NOT NULL DEFAULT 'running',
    summary    TEXT NOT NULL DEFAULT '',
    error      TEXT NOT NULL DEFAULT '',
    started_at TEXT NOT NULL,
    ended_at   TEXT
);"#,
        r#"CREATE TABLE IF NOT EXISTS path_locks (
    id         TEXT PRIMARY KEY,
    task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    path       TEXT NOT NULL UNIQUE,
    expires_at TEXT,
    created_at TEXT NOT NULL
);"#,
        r#"CREATE TABLE IF NOT EXISTS checkpoints (
    id               TEXT PRIMARY KEY,
    task_id          TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    status           TEXT NOT NULL DEFAULT 'open',
    conversation_ref TEXT NOT NULL DEFAULT '',
    created_at       TEXT NOT NULL,
    updated_at       TEXT NOT NULL
);"#,
        r#"CREATE TABLE IF NOT EXISTS artifacts (
    id         TEXT PRIMARY KEY,
    task_id    TEXT NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    run_id     TEXT REFERENCES task_runs(id) ON DELETE SET NULL,
    kind       TEXT NOT NULL,
    path       TEXT NOT NULL DEFAULT '',
    content    TEXT NOT NULL DEFAULT '',
    created_at TEXT NOT NULL
);"#,
        "CREATE INDEX IF NOT EXISTS idx_blueprint_items_blueprint_id ON blueprint_items (blueprint_id);",
        "CREATE INDEX IF NOT EXISTS idx_tasks_blueprint_item_id ON tasks (blueprint_item_id);",
        "CREATE INDEX IF NOT EXISTS idx_tasks_parent_id ON tasks (parent_id);",
        "CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks (status);",
        "CREATE INDEX IF NOT EXISTS idx_task_steps_task_id ON task_steps (task_id);",
        "CREATE INDEX IF NOT EXISTS idx_task_runs_task_id ON task_runs (task_id);",
        "CREATE INDEX IF NOT EXISTS idx_checkpoints_task_id ON checkpoints (task_id);",
        "CREATE INDEX IF NOT EXISTS idx_artifacts_task_id ON artifacts (task_id);",
    ]
}

fn drop_statements() -> &'static [&'static str] {
    &[
        "DROP TABLE IF EXISTS artifacts;",
        "DROP TABLE IF EXISTS checkpoints;",
        "DROP TABLE IF EXISTS path_locks;",
        "DROP TABLE IF EXISTS task_runs;",
        "DROP TABLE IF EXISTS task_steps;",
        "DROP TABLE IF EXISTS tasks;",
        "DROP TABLE IF EXISTS blueprint_items;",
        "DROP TABLE IF EXISTS blueprints;",
    ]
}
