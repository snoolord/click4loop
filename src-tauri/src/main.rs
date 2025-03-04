// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// src-tauri/src/main.rs

#[taurpc::procedures]
trait Api {
    async fn greet(name: String) -> String;
}

#[derive(Clone)]
struct ApiImpl;

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn greet(self, name: String) -> String {
        format!("Hello, {}!", name)
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(taurpc::create_ipc_handler(ApiImpl.into_handler()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
