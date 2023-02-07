use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, TS)]
#[sea_orm(table_name = "reader_block")]
#[ts(export, rename = "ReaderBlock")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub config: String, // ReaderBlockConfig
    pub reader_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::reader::Entity",
        from = "Column::ReaderId",
        to = "super::reader::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Reader,
}

impl Related<super::reader::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reader.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
