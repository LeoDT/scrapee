use crate::dao::entities;
use serde::{Deserialize, Serialize};

pub trait FromModel<T> {
    fn from_model(model: T) -> Self;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SitesResponse {
    sites: Vec<entities::site::Model>,
}

impl FromModel<Vec<entities::site::Model>> for SitesResponse {
    fn from_model(sites: Vec<entities::site::Model>) -> Self {
        Self { sites }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageResponse {
    page: entities::page::Model,
    fields: Vec<entities::field::Model>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageContentsResponse {
    page_contents: Vec<entities::page_content::Model>,
}

impl FromModel<Vec<entities::page_content::Model>> for PageContentsResponse {
    fn from_model(page_contents: Vec<entities::page_content::Model>) -> Self {
        Self { page_contents }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
