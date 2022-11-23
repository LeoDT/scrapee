#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("crawl error {0}")]
    CrawlError(#[from] reqwest::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error("url pattern parse error: {0}")]
    UrlPatternParseError(String),

    #[error("unknown error")]
    Unknown,
}
