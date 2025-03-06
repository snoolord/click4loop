// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// src-tauri/src/main.rs

use click4loop_tauri_lib::mouse_listener::{
    start_mouse_listener, stop_mouse_listener, MouseEvent, MouseListenerState,
};

#[taurpc::procedures]
trait Api {
    async fn greet(name: String) -> String;
    async fn start_mouse_listener();
    async fn stop_mouse_listener();
}

#[derive(Clone)]
struct ApiImpl {
    mouse_state: MouseListenerState,
}

#[taurpc::resolvers]
impl Api for ApiImpl {
    async fn greet(self, name: String) -> String {
        format!("Hello, {}!", name)
    }
    async fn start_mouse_listener(self) {
        let mouse_state = self.mouse_state.clone();
        start_mouse_listener(mouse_state, |event: MouseEvent| {
            println!(
                "Event received: {}, {} x, {}, y",
                event.button.unwrap(),
                event.x,
                event.y
            );
        })
        .await;
    }

    async fn stop_mouse_listener(self) {
        let mouse_state = self.mouse_state.clone();
        stop_mouse_listener(mouse_state).await;
    }
}

#[tokio::main]
async fn main() {
    let api_handler = ApiImpl {
        mouse_state: MouseListenerState::new(),
    }
    .into_handler(); // This generates a TauRPC handler type.

    tauri::Builder::default()
        .invoke_handler(taurpc::create_ipc_handler(api_handler))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
