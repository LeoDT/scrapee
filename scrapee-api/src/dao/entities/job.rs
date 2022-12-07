use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::job::{JobKind, JobStatus};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "job")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub kind: JobKind,
    pub message: String,
    pub status: JobStatus,
    pub fail_message: Option<String>,
    pub fail_attempts: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub started_at: Option<DateTime>,
    pub successed_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
