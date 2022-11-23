use std::sync::Arc;

use wax::{Glob, Pattern};

use crate::error;

#[derive(Debug, Clone)]
pub struct Page {
    pub name: String,
    pub url: Option<String>,
    pub url_pattern: Glob<'static>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub xpath: String,

    pub try_follow: bool,
    pub group_to: Option<String>,
}

pub struct Site {
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

    pub fn get_page_for_url(&self, url: String) -> Option<Arc<Page>> {
        let url = url.replace("http://", "").replace("https://", "");

        self.pages
            .iter()
            .find(|p| p.url_pattern.is_match(url.as_str()))
            .cloned()
    }
}

pub fn make_url_pattern(pattern_str: &str) -> Result<Glob, error::Error> {
    let replaces: Vec<(&str, &str)> = vec![
        ("{{page}}", "<[0-9]:1,>"),
        ("{{id}}", "*"),
        ("http://", ""),
        ("https://", ""),
    ];

    let pattern = replaces
        .iter()
        .fold(pattern_str.to_string(), |s, (from, to)| s.replace(from, to));

    if let Ok(result) = Glob::new(pattern.as_str()) {
        Ok(result.into_owned())
    } else {
        Err(error::Error::UrlPatternParseError(pattern_str.to_string()))
    }
}
