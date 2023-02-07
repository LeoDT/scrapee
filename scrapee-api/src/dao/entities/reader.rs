use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, TS)]
#[sea_orm(table_name = "reader")]
#[ts(export, rename = "Reader")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::reader_block::Entity")]
    ReaderBlock,
}

impl Related<super::reader_block::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReaderBlock.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
