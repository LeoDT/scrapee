use thiserror::Error;

pub type ScrapeeResult<T> = Result<T, ScrapeeError>;
pub type ScrapeeDbResult<T> = Result<T, ScrapeeDbError>;

#[derive(Error, Debug)]
pub enum ScrapeeError {
    #[error("crawl error {0}")]
    CrawlError(#[from] reqwest::Error),

    #[error(transparent)]
    DeserializationError(#[from] serde_json::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    UrlPatternParseError(#[from] regex::Error),

    #[error(transparent)]
    DbError(#[from] ScrapeeDbError),

    #[error("unknown error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum ScrapeeDbError {
    #[error(transparent)]
    MigrationError(#[from] sqlx::Error),

    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    SerializationError(#[from] serde_json::Error),

    #[error("no {0} found in table {1}")]
    NotExist(i32, String),
}
