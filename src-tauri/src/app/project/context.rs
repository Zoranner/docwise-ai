use std::path::{Path, PathBuf};

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};

/// 工作区 `.agent/project.db` 连接与崩溃恢复。
pub struct ProjectContext {
    workspace_root: PathBuf,
    pub db: DatabaseConnection,
}

impl ProjectContext {
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub async fn open(workspace_root: PathBuf) -> Result<Self, DbErr> {
        let agent_dir = workspace_root.join(".agent");
        tokio::fs::create_dir_all(&agent_dir)
            .await
            .map_err(|e| DbErr::Custom(format!("create .agent: {e}")))?;

        let db_path = agent_dir.join("project.db");
        let url = sqlite_url(&db_path)?;
        let db = Database::connect(&url).await?;
        Migrator::up(&db, None).await?;

        let ctx = Self { workspace_root, db };
        ctx.recover_after_restart().await?;
        Ok(ctx)
    }

    /// 应用启动恢复：遗留 running 任务/运行实例、过期路径锁。
    async fn recover_after_restart(&self) -> Result<(), DbErr> {
        self.db
            .execute(Statement::from_string(
                DbBackend::Sqlite,
                r#"UPDATE tasks
SET status = 'blocked', blocked_reason = 'process_restart', updated_at = datetime('now')
WHERE status = 'running';"#
                    .to_owned(),
            ))
            .await?;

        self.db
            .execute(Statement::from_string(
                DbBackend::Sqlite,
                r#"UPDATE task_runs
SET status = 'failed',
    error = 'process_restart',
    ended_at = COALESCE(ended_at, datetime('now'))
WHERE status = 'running';"#
                    .to_owned(),
            ))
            .await?;

        self.db
            .execute(Statement::from_string(
                DbBackend::Sqlite,
                r#"DELETE FROM path_locks
WHERE expires_at IS NOT NULL
  AND expires_at != ''
  AND expires_at < datetime('now');"#
                    .to_owned(),
            ))
            .await?;

        Ok(())
    }
}

fn sqlite_url(path: &Path) -> Result<String, DbErr> {
    let normalized = path.to_string_lossy().replace('\\', "/");
    Ok(format!("sqlite://{normalized}?mode=rwc"))
}
