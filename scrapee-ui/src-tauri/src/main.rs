#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use scrapee_api::{
    app_state::{connect_db, AppContext},
    tauri_plugin::ScrapeePlugin,
};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let db = connect_db("sqlite:a.db").await.unwrap();

    let app_context = AppContext::new(db);
    // scrapee_api::dao::seed_data::saraba(app_context.clone()).await;

    tauri::Builder::default()
        .plugin(ScrapeePlugin::new(app_context))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
