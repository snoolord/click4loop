// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// src-tauri/src/main.rs

use click4loop_tauri_lib::mouse_listener::{
    playback_events, start_mouse_listener, start_playback_loop, stop_mouse_listener,
    stop_playback_loop, MouseEvent, MouseListenerState,
};
use tauri::{TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

#[taurpc::procedures]
trait Api {
    async fn greet(name: String) -> String;
    async fn start_mouse_listener();
    async fn stop_mouse_listener();
    async fn playback_events();
    async fn start_playback(loop_playback: bool);
    async fn stop_playback();
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
            if let Some(button) = event.button {
                println!(
                    "Event received: {:?}, {} x, {}, y",
                    button, event.x, event.y
                );
            }
        })
        .await;
    }

    async fn stop_mouse_listener(self) {
        let mouse_state = self.mouse_state.clone();
        stop_mouse_listener(mouse_state).await;
    }

    async fn playback_events(self) {
        let mouse_state = self.mouse_state.clone();
        if let Err(e) = playback_events(mouse_state).await {
            eprintln!("Error during playback_events: {:?}", e);
        }
    }
    async fn start_playback(self, loop_playback: bool) {
        let mouse_state = self.mouse_state.clone();
        if loop_playback {
            start_playback_loop(mouse_state).await;
        } else {
            if let Err(e) = playback_events(mouse_state).await {
                eprintln!("Error during playback_events: {:?}", e);
            }
        }
    }

    async fn stop_playback(self) {
        let mouse_state = self.mouse_state.clone();
        stop_playback_loop(mouse_state).await;
    }
}

#[tokio::main]
async fn main() {
    let api_handler = ApiImpl {
        mouse_state: MouseListenerState::new(),
    }
    .into_handler(); // This generates a TauRPC handler type.

    tauri::Builder::default()
        .setup(|app| {
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                .title("click4loop")
                .inner_size(200.0, 150.0);

            // set transparent title bar only when building for macOS
            #[cfg(target_os = "macos")]
            let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

            let window = win_builder.build().unwrap();

            // set background color only when building for macOS
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSColor, NSWindow};
                use cocoa::base::{id, nil};

                let ns_window = window.ns_window().unwrap() as id;
                unsafe {
                    let bg_color = NSColor::colorWithRed_green_blue_alpha_(
                        nil,
                        52.0 / 255.0,
                        48.0 / 255.0,
                        47.0 / 255.0,
                        1.0, // Fully opaque
                    );
                    ns_window.setBackgroundColor_(bg_color);
                }
            }

            Ok(())
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(taurpc::create_ipc_handler(api_handler))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
