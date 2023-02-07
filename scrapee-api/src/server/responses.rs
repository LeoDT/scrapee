use crate::dao::entities;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub trait FromModel<T> {
    fn from_model(model: T) -> Self;
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts[export]]
pub struct SiteResponse {
    site: entities::site::Model,
}

impl FromModel<entities::site::Model> for SiteResponse {
    fn from_model(site: entities::site::Model) -> Self {
        Self { site }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct SitesResponse {
    sites: Vec<entities::site::Model>,
}

impl FromModel<Vec<entities::site::Model>> for SitesResponse {
    fn from_model(sites: Vec<entities::site::Model>) -> Self {
        Self { sites }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct PageResponse {
    page: entities::page::Model,
    fields: Vec<entities::field::Model>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct PagesResponse {
    pages: Vec<PageResponse>,
}

impl FromModel<Vec<(entities::page::Model, Vec<entities::field::Model>)>> for PagesResponse {
    fn from_model(m: Vec<(entities::page::Model, Vec<entities::field::Model>)>) -> Self {
        Self {
            pages: m
                .into_iter()
                .map(|(page, fields)| PageResponse { page, fields })
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct PageContentsResponse {
    page_contents: Vec<entities::page_content::Model>,
}

impl FromModel<Vec<entities::page_content::Model>> for PageContentsResponse {
    fn from_model(page_contents: Vec<entities::page_content::Model>) -> Self {
        Self { page_contents }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct JobResponse {
    job: entities::job::Model,
}

impl FromModel<entities::job::Model> for JobResponse {
    fn from_model(job: entities::job::Model) -> Self {
        Self { job }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JobsResponse {
    jobs: Vec<entities::job::Model>,
}

impl FromModel<Vec<entities::job::Model>> for JobsResponse {
    fn from_model(jobs: Vec<entities::job::Model>) -> Self {
        Self { jobs }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ReadersResponse {
    readers: Vec<ReaderResponse>,
}

impl FromModel<Vec<(entities::reader::Model, Vec<entities::reader_block::Model>)>>
    for ReadersResponse
{
    fn from_model(
        readers: Vec<(entities::reader::Model, Vec<entities::reader_block::Model>)>,
    ) -> Self {
        Self {
            readers: readers
                .into_iter()
                .map(ReaderResponse::from_model)
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ReaderResponse {
    reader: entities::reader::Model,
    blocks: Vec<entities::reader_block::Model>,
}

impl FromModel<(entities::reader::Model, Vec<entities::reader_block::Model>)> for ReaderResponse {
    fn from_model(
        (reader, blocks): (entities::reader::Model, Vec<entities::reader_block::Model>),
    ) -> Self {
        Self { reader, blocks }
    }
}

impl FromModel<entities::reader::Model> for ReaderResponse {
    fn from_model(reader: entities::reader::Model) -> Self {
        Self {
            reader,
            blocks: vec![],
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ReaderBlockResponse {
    block: entities::reader_block::Model,
}

impl FromModel<entities::reader_block::Model> for ReaderBlockResponse {
    fn from_model(block: entities::reader_block::Model) -> Self {
        Self { block }
    }
}
