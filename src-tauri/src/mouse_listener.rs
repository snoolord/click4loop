use rdev::{listen, EventType};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Shared state for managing the mouse listener thread.
#[derive(Clone)]
pub struct MouseListenerState {
    pub stop_flag: Arc<AtomicBool>, // Flag to signal thread termination
    pub thread_handle: Arc<Mutex<Option<std::thread::JoinHandle<()>>>>, // Handle to manage the thread
}

impl MouseListenerState {
    /// Creates a new instance of the shared state.
    pub fn new() -> Self {
        Self {
            stop_flag: Arc::new(AtomicBool::new(false)),
            thread_handle: Arc::new(Mutex::new(None)),
        }
    }
}
#[derive(Debug)]
pub struct MouseEvent {
    pub x: f64,
    pub y: f64,
    pub button: Option<String>,
}

/// Starts the mouse listener in a separate thread.
pub async fn start_mouse_listener<F>(state: MouseListenerState, emit: F)
where
    F: Fn(MouseEvent) + Send + 'static,
{
    let stop_flag = Arc::clone(&state.stop_flag);

    // Check if a thread is already running
    let mut handle_guard = state.thread_handle.lock().await;
    if handle_guard.is_some() {
        println!("Mouse listener is already running.");
        return;
    }

    // Reset the stop flag to allow restarting
    stop_flag.store(false, Ordering::Relaxed);

    println!("Mouse listener started.");
    // Spawn the listener thread
    let handle = thread::spawn(move || {
        let debounce_duration = Duration::from_millis(50);
        let mut last_event_time = Instant::now();
        let mut last_event_type: Option<EventType> = None;
        let mut x_coordinate: f64 = 0.0;
        let mut y_coordinate: f64 = 0.0;

        if let Err(error) = listen(move |event| {
            if stop_flag.load(Ordering::Relaxed) {
                return; // Exit if stop flag is set
            }

            let now = Instant::now();
            let elapsed_time = now.duration_since(last_event_time);

            if elapsed_time > debounce_duration || Some(event.event_type.clone()) != last_event_type
            {
                match event.event_type {
                    EventType::ButtonPress(btn) => {
                        emit(MouseEvent {
                            x: x_coordinate,
                            y: y_coordinate,
                            button: Some(format!("{:?}", btn)),
                        });
                    }
                    EventType::ButtonRelease(btn) => {
                        emit(MouseEvent {
                            x: x_coordinate,
                            y: y_coordinate,
                            button: Some(format!("{:?}", btn)),
                        });
                    }
                    EventType::MouseMove { x: new_x, y: new_y } => {
                        x_coordinate = new_x;
                        y_coordinate = new_y;
                    }
                    _ => {}
                }

                last_event_type = Some(event.event_type.clone());
                last_event_time = now;
            }
        }) {
            eprintln!("Error: {:?}", error);
        }
    });

    *handle_guard = Some(handle);
}

/// Stops the mouse listener by signaling the thread to terminate.
pub async fn stop_mouse_listener(state: MouseListenerState) {
    // Signal the thread to stop
    state.stop_flag.store(true, Ordering::Relaxed);

    // Wait for the thread to finish
    let mut handle_guard = state.thread_handle.lock().await;
    if let Some(handle) = handle_guard.take() {
        if let Err(err) = handle.join() {
            eprintln!("Error joining listener thread: {:?}", err);
        }
        println!("Mouse listener stopped.");
    } else {
        println!("No mouse listener is running.");
    }
}
