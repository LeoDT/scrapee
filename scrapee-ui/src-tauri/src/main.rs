#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use scrapee_api::{
    app_state::{connect_db, AppContext, AppState},
    dao::seed_data::saraba,
    tauri_plugin::ScrapeePlugin,
};

#[tauri::command]
fn greet(app_state: tauri::State<AppState>, name: &str) -> String {
    app_state.test_message();

    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let db = connect_db("sqlite:a.db").await.unwrap();
    let _ = saraba(db.clone()).await;

    let app_context = AppContext::new(db);

    tauri::Builder::default()
        .plugin(ScrapeePlugin::new(app_context))
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
