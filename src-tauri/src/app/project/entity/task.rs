use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "blueprint_item_id")]
    pub blueprint_item_id: Option<String>,
    #[sea_orm(column_name = "parent_id")]
    pub parent_id: Option<String>,
    #[sea_orm(column_name = "conversation_ref")]
    pub conversation_ref: String,
    pub title: String,
    pub goal: String,
    pub acceptance: String,
    pub status: String,
    pub priority: i32,
    #[sea_orm(column_name = "blocked_reason")]
    pub blocked_reason: String,
    pub tags: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
