use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};

use crate::{
    dao::{run_migrate, DaoProvider},
    error::ScrapeeDbResult,
    message::MessageCenter,
};

#[derive(Clone)]
pub struct AppContext {
    pub db: DatabaseConnection,
    pub message_center: Arc<MessageCenter>,
}

impl AppContext {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            message_center: Arc::new(MessageCenter::new()),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    app_context: AppContext,
}

impl AppState {
    pub fn new(app_context: AppContext) -> Self {
        Self { app_context }
    }

    pub fn test_message(&self) {
        let _ = self
            .app_context
            .message_center
            .tx()
            .send(crate::message::Message::JobCreated { job_id: 2 });
    }
}

impl DaoProvider for AppState {
    fn dao_app_context(&self) -> AppContext {
        self.app_context.clone()
    }
}

pub async fn connect_db(db_uri: &str) -> ScrapeeDbResult<DatabaseConnection> {
    let _ = run_migrate(db_uri).await?;
    let db = Database::connect(db_uri).await?;

    Ok(db)
}
