use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "blueprint_items")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    #[sea_orm(column_name = "blueprint_id")]
    pub blueprint_id: String,
    pub seq: i32,
    #[sea_orm(column_name = "file_path")]
    pub file_path: String,
    pub title: String,
    pub audience: String,
    pub goal: String,
    #[sea_orm(column_name = "must_cover")]
    pub must_cover: String,
    pub constraints: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
