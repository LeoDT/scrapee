use tauri::plugin::Plugin;
use tauri::{AppHandle, Invoke, Manager, Runtime};

use crate::app_state::{AppContext, AppState};
use crate::dao::seed_data::saraba;
use crate::job::manager::JobManager;
use crate::server::serve;

#[tauri::command]
async fn start_job(app_state: tauri::State<'_, AppState>) -> Result<(), ()> {
    log::info!("start");

    let _ = app_state.test_message().await;

    Ok(())
}

#[tauri::command]
async fn seed_saraba(app_state: tauri::State<'_, AppState>) -> Result<(), ()> {
    let _ = saraba(app_state.app_context.clone()).await;

    Ok(())
}

pub struct ScrapeePlugin<R: Runtime> {
    app_context: AppContext,
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> ScrapeePlugin<R> {
    pub fn new(app_context: AppContext) -> Self {
        Self {
            app_context,
            invoke_handler: Box::new(tauri::generate_handler![start_job, seed_saraba]),
        }
    }
}

impl<R: Runtime> Plugin<R> for ScrapeePlugin<R> {
    fn name(&self) -> &'static str {
        "scrapee"
    }

    fn initialize(
        &mut self,
        app: &AppHandle<R>,
        _config: serde_json::value::Value,
    ) -> tauri::plugin::Result<()> {
        let app_state = AppState::new(self.app_context.clone());
        app.manage(app_state.clone());

        let job_manager = JobManager::new(self.app_context.clone());
        job_manager.run();

        app.manage(job_manager);

        let _ = serve(self.app_context.clone());

        Ok(())
    }

    fn initialization_script(&self) -> Option<String> {
        Some(format!(
            r###"window.__SCRAPEE_CONFIG__ = {{ port: {},token: '{}' }};"###,
            self.app_context.server_port, self.app_context.server_client_token
        ))
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        log::info!("extend api");

        (self.invoke_handler)(message)
    }
}
