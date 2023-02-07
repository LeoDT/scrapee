use serde::Deserialize;

use crate::{
    job::{JobKind, JobMessage},
    reader::ReaderBlockConfig,
};

#[derive(Deserialize)]
pub struct CreateJobRequest {
    pub kind: JobKind,
    pub message: JobMessage,
}

#[derive(Deserialize)]
pub struct CreateReaderRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateReaderBlockRequest {
    pub config: ReaderBlockConfig,
}
