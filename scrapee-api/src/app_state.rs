use std::sync::Arc;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::{
    dao::{run_migrate, DaoProvider},
    error::{ScrapeeDbResult, ScrapeeResult},
    job::JobKind,
    message::MessageCenter,
};

#[derive(Clone)]
pub struct AppContext {
    pub db: DatabaseConnection,
    pub message_center: Arc<MessageCenter>,
    pub server_port: u16,
    pub server_client_token: &'static str,
}

#[cfg(debug_assertions)]
fn get_port() -> u16 {
    3333
}

#[cfg(not(debug_assertions))]
fn get_port() -> u16 {
    portpicker::pick_unused_port().expect("no free port")
}

impl AppContext {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            message_center: Arc::new(MessageCenter::new()),
            server_port: get_port(), // TODO production error handling
            server_client_token: "test",
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub app_context: AppContext,
}

impl AppState {
    pub fn new(app_context: AppContext) -> Self {
        Self { app_context }
    }

    pub async fn test_message(&self) -> ScrapeeResult<()> {
        let m = self
            .dao()
            .add_job(
                JobKind::Collect,
                crate::job::JobMessage::Collect { site_id: 1 },
            )
            .await?;

        log::info!("{:?}", m);

        let _ = self
            .app_context
            .message_center
            .tx()
            .send(crate::message::Message::JobCreated { job_id: m.id });

        Ok(())
    }
}

impl DaoProvider for AppState {
    fn dao_app_context(&self) -> AppContext {
        self.app_context.clone()
    }
}

pub async fn connect_db(db_uri: &str) -> ScrapeeDbResult<DatabaseConnection> {
    let _ = run_migrate(db_uri).await?;

    let mut opt = ConnectOptions::new(db_uri.to_owned());

    opt.sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;

    Ok(db)
}
