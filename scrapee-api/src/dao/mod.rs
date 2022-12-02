use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::*;
use sqlx::migrate::MigrateDatabase;

use crate::app_state::AppContext;
use crate::collector::CollectedContent;
use crate::error::{ScrapeeDbError, ScrapeeDbResult};
use crate::job::{JobKind, JobMessage};

pub mod entities;
pub mod seed_data;

use entities::*;

pub trait DaoProvider {
    fn dao_app_context(&self) -> AppContext;

    fn dao(&self) -> Dao {
        Dao::new(self.dao_app_context())
    }
}

#[derive(Clone)]
pub struct Dao {
    pub db: DatabaseConnection,
}

impl Dao {
    pub fn new(app_context: AppContext) -> Self {
        Self { db: app_context.db }
    }

    pub async fn add_site(&self, name: String, save_context: bool) -> ScrapeeDbResult<site::Model> {
        let d = Utc::now().naive_utc();

        let s = site::ActiveModel {
            name: Set(name),
            save_context: Set(save_context),

            created_at: Set(d),
            updated_at: Set(d),

            ..Default::default()
        };

        Ok(s.insert(&self.db).await?)
    }

    pub async fn get_site_by_id(&self, id: i32) -> ScrapeeDbResult<site::Model> {
        match site::Entity::find_by_id(id).one(&self.db).await? {
            Some(m) => Ok(m),
            None => Err(ScrapeeDbError::NotExist(id, "site".to_string())),
        }
    }

    pub async fn add_page(
        &self,
        name: String,
        url_pattern: String,
        site_id: i32,
        url: Option<String>,
    ) -> ScrapeeDbResult<page::Model> {
        let d = Utc::now().naive_utc();

        let p = page::ActiveModel {
            name: Set(name),
            url_pattern: Set(url_pattern),
            site_id: Set(site_id),
            url: Set(url),

            created_at: Set(d),
            updated_at: Set(d),

            ..Default::default()
        };

        Ok(p.insert(&self.db).await?)
    }

    pub async fn get_page_by_id(&self, id: i32) -> ScrapeeDbResult<page::Model> {
        match page::Entity::find_by_id(id).one(&self.db).await? {
            Some(m) => Ok(m),
            None => Err(ScrapeeDbError::NotExist(id, "page".to_string())),
        }
    }

    pub async fn find_pages_by_site_id_with_fields(
        &self,
        site_id: i32,
    ) -> ScrapeeDbResult<Vec<(page::Model, Vec<field::Model>)>> {
        Ok(page::Entity::find()
            .filter(page::Column::SiteId.eq(site_id))
            .find_with_related(field::Entity)
            .all(&self.db)
            .await?)
    }

    pub async fn add_field(
        &self,
        name: String,
        xpath: String,
        try_follow: bool,
        page_id: i32,
        group_to: Option<String>,
    ) -> ScrapeeDbResult<field::Model> {
        let d = Utc::now().naive_utc();

        let m = field::ActiveModel {
            name: Set(name),
            xpath: Set(xpath),
            try_follow: Set(try_follow),
            page_id: Set(page_id),
            group_to: Set(group_to),

            created_at: Set(d),
            updated_at: Set(d),

            ..Default::default()
        };

        Ok(m.insert(&self.db).await?)
    }

    pub async fn add_page_content(
        &self,
        page_id: i32,
        url: String,
        content: CollectedContent,
        is_archive: bool,
    ) -> ScrapeeDbResult<page_content::Model> {
        let d = Utc::now().naive_utc();

        let m = page_content::ActiveModel {
            content: Set(serde_json::to_string(&content)?),
            url,
            page_id: Set(page_id),
            is_archive: Set(is_archive),

            created_at: Set(d),
            updated_at: Set(d),

            ..Default::default()
        };

        Ok(m.insert(&self.db).await?)
    }

    pub async fn add_job(&self, kind: JobKind, message: JobMessage) -> ScrapeeDbResult<job::Model> {
        let d = Utc::now().naive_utc();

        let m = job::ActiveModel {
            kind: Set(kind),
            message: Set(serde_json::to_string(&message)?),

            created_at: Set(d),
            updated_at: Set(d),

            ..Default::default()
        };

        Ok(m.insert(&self.db).await?)
    }

    pub async fn finish_job(&self, job_id: i32) -> ScrapeeDbResult<job::ActiveModel> {
        let m = self.get_job_by_id(job_id).await?;
        let am: job::ActiveModel = m.into();

        am.status = Set(crate::job::JobStatus::Success);

        am.update(&self.db).await?;

        am
    }

    pub async fn get_job_by_id(&self, id: i32) -> ScrapeeDbResult<job::Model> {
        match job::Entity::find_by_id(id).one(&self.db).await? {
            Some(m) => Ok(m),
            None => Err(ScrapeeDbError::NotExist(id, "job".to_string())),
        }
    }

    pub async fn find_jobs(&self) -> ScrapeeDbResult<Vec<job::Model>> {
        Ok(job::Entity::find().all(&self.db).await?)
    }
}

pub async fn run_migrate(uri: &str) -> Result<(), sqlx::error::Error> {
    use sqlx::Connection;
    if !sqlx::Sqlite::database_exists(uri).await? {
        sqlx::Sqlite::create_database(uri).await?;
    }
    let mut conn = sqlx::SqliteConnection::connect(uri).await?;
    sqlx::migrate!().run(&mut conn).await?;

    Ok(())
}
