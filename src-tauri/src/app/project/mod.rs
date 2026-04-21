mod context;
pub mod dto;
pub mod entity;
mod ops;
pub mod params;
pub mod tools;
mod util;

pub use context::ProjectContext;
pub use ops::*;
pub use params::{BlueprintItemAddParams, BlueprintItemUpdateParams, TaskUpdateParams};

#[cfg(test)]
mod tests {
    use super::ProjectContext;
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn project_context_creates_review_and_output_tables() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("docwise-project-test-{nanos}"));
        tokio::fs::create_dir_all(&root).await.expect("create root");

        let ctx = ProjectContext::open(root.clone())
            .await
            .expect("open project context");

        let rows = ctx
            .db
            .query_all(Statement::from_string(
                DbBackend::Sqlite,
                "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name".to_owned(),
            ))
            .await
            .expect("query sqlite_master");

        let names: Vec<String> = rows
            .iter()
            .filter_map(|row| row.try_get::<String>("", "name").ok())
            .collect();

        assert!(names.iter().any(|name| name == "reviews"));
        assert!(names.iter().any(|name| name == "outputs"));
        assert!(!names.iter().any(|name| name == "checkpoints"));
        assert!(!names.iter().any(|name| name == "artifacts"));

        tokio::fs::remove_dir_all(root).await.expect("cleanup root");
    }
}
