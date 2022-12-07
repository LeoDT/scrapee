use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "page")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub url_pattern: String,
    pub site_id: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::site::Entity",
        from = "Column::SiteId",
        to = "super::site::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Site,
    #[sea_orm(has_many = "super::field::Entity")]
    Field,
    #[sea_orm(has_many = "super::page_content::Entity")]
    PageContent,
}

impl Related<super::site::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Site.def()
    }
}

impl Related<super::field::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Field.def()
    }
}

impl Related<super::page_content::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PageContent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
