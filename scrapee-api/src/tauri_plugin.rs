use tauri::plugin::Plugin;
use tauri::{AppHandle, Invoke, Manager, Runtime};

use crate::app_state::{AppContext, AppState};
use crate::job::manager::JobManager;

pub struct ScrapeePlugin<R: Runtime> {
    app_context: AppContext,
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> ScrapeePlugin<R> {
    pub fn new(app_context: AppContext) -> Self {
        Self {
            app_context,
            invoke_handler: Box::new(tauri::generate_handler![]),
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

        Ok(())
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
