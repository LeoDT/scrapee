use serde::Deserialize;

use crate::job::{JobKind, JobMessage};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub kind: JobKind,
    pub message: JobMessage,
}
