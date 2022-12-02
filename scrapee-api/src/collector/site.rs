use std::sync::Arc;

use regex::Regex;

use crate::{dao::Dao, error::ScrapeeResult};

#[derive(Debug, Clone)]
pub struct Page {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub url_pattern: Regex,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub id: i32,
    pub name: String,
    pub xpath: String,

    pub try_follow: bool,
    pub group_to: Option<String>,
}

pub struct Site {
    pub id: i32,
    pub name: String,
    pub save_context: bool,

    pub pages: Vec<Arc<Page>>,
}

impl Site {
    pub fn get_start_urls(&self) -> Vec<String> {
        self.pages
            .iter()
            .filter(|p| p.url.is_some())
            .map(|p| p.url.clone().unwrap())
            .collect()
    }

    pub fn find_page_for_url(&self, url: String) -> Option<Arc<Page>> {
        self.pages
            .iter()
            .find(|p| p.url_pattern.is_match(url.as_str()))
            .cloned()
    }
}

pub fn make_url_pattern(pattern_str: String) -> ScrapeeResult<Regex> {
    let replaces: Vec<(&str, &str)> = vec![("{{page}}", "[0-9]+"), ("{{id}}", ".+")];

    let pattern = replaces
        .iter()
        .fold(pattern_str.to_string(), |s, (from, to)| s.replace(from, to));

    Ok(Regex::new(pattern.as_str())?)
}

pub async fn make_site_by_id(id: i32, dao: Dao) -> ScrapeeResult<Site> {
    let site = dao.get_site_by_id(id).await?;
    let pages = dao.find_pages_by_site_id_with_fields(site.id).await?;

    let pages = pages
        .into_iter()
        .map(|(p, fields)| {
            let fields = fields
                .into_iter()
                .map(|f| Field {
                    id: f.id,
                    name: f.name,
                    xpath: f.xpath,
                    try_follow: f.try_follow,
                    group_to: f.group_to,
                })
                .collect();

            Arc::new(Page {
                id: p.id,
                name: p.name,
                url: p.url,
                url_pattern: make_url_pattern(p.url_pattern).unwrap(),
                fields,
            })
        })
        .collect();

    Ok(Site {
        id: site.id,
        name: site.name,
        save_context: site.save_context,
        pages,
    })
}
