use serde::{Deserialize, Serialize};

use sea_orm::{DeriveActiveEnum, EnumIter};

use crate::{
    dao::entities::job::Model as JobModel,
    error::{ScrapeeError, ScrapeeResult},
};

pub mod manager;

#[derive(Clone, Debug, Eq, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum JobStatus {
    Waiting = 0,
    Running = 1,
    Success = 2,
    Failed = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum JobKind {
    Collect = 0,
    PersistMedia = 1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JobMessage {
    Collect { site_id: i32 },
    PersistMedia { page_content_id: i32 },
}

pub struct Job {
    id: i32,
    kind: JobKind,
    message: JobMessage,
    status: JobStatus,
}

impl TryFrom<JobModel> for Job {
    type Error = ScrapeeError;

    fn try_from(m: JobModel) -> ScrapeeResult<Self> {
        Ok(Job {
            id: m.id,
            kind: m.kind,
            status: m.status,
            message: serde_json::from_str::<JobMessage>(&m.message)?,
        })
    }
}
