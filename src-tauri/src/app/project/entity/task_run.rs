use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "task_runs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "task_id")]
    pub task_id: String,
    pub status: String,
    pub summary: String,
    pub error: String,
    #[sea_orm(column_name = "started_at")]
    pub started_at: String,
    #[sea_orm(column_name = "ended_at")]
    pub ended_at: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
