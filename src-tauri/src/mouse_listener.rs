use rdev::{simulate, Button, EventType, SimulateError};
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Struct to hold mouse event data.
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: f64,
    pub y: f64,
    pub button: Option<Button>,
}

/// Shared state for managing the mouse listener thread.
#[derive(Clone, Debug)]
pub struct MouseListenerState {
    pub stop_flag: Arc<AtomicBool>, // Flag to signal thread termination
    pub playback_flag: Arc<AtomicBool>, // Flag to control playback loop
    pub thread_handle: Arc<Mutex<Option<std::thread::JoinHandle<()>>>>, // Handle to manage the thread
    pub playback_thread_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>, // Handle to manage the playback thread
    pub event_queue: Arc<Mutex<VecDeque<MouseEvent>>>, // Queue to store mouse events
    pub last_event_played: Arc<Mutex<Option<MouseEvent>>>, // Last event played during playback
}

impl MouseListenerState {
    /// Creates a new instance of the shared state.
    pub fn new() -> Self {
        Self {
            stop_flag: Arc::new(AtomicBool::new(false)),
            playback_flag: Arc::new(AtomicBool::new(false)),
            thread_handle: Arc::new(Mutex::new(None)),
            playback_thread_handle: Arc::new(Mutex::new(None)),
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
            last_event_played: Arc::new(Mutex::new(None)), // Initialize with None
        }
    }
    pub async fn reset_last_event_played(&self) {
        let mut last_event = self.last_event_played.lock().await;
        *last_event = None;
    }
}

/// Starts the mouse listener in a separate thread.
pub async fn start_mouse_listener<F>(state: MouseListenerState, emit: F)
where
    F: Fn(MouseEvent) + Send + 'static,
{
    let stop_flag = Arc::clone(&state.stop_flag);
    let event_queue = Arc::clone(&state.event_queue);

    // Check if a thread is already running
    let mut handle_guard = state.thread_handle.lock().await;
    if handle_guard.is_some() {
        println!("Mouse listener is already running.");
        return;
    }

    // Clear the event queue
    {
        let mut queue = event_queue.lock().await;
        queue.clear();
    }

    // Reset the stop flag to allow restarting
    stop_flag.store(false, Ordering::Relaxed);

    println!("Mouse listener started.");
    // Spawn the listener thread
    let coordinates = Arc::new(Mutex::new((0.0f64, 0.0f64)));
    let last_event_data = Arc::new(Mutex::new((Instant::now(), None::<EventType>)));

    let coordinates_clone = Arc::clone(&coordinates);
    let last_event_data_clone = Arc::clone(&last_event_data);

    let handle = thread::spawn(move || {
        let debounce_duration = Duration::from_millis(50);

        #[cfg(target_os = "macos")]
        rdev::set_is_main_thread(false);
        #[cfg(target_os = "windows")]
        rdev::set_event_popup(false);

        let func = move |event: rdev::Event| -> Option<rdev::Event> {
            if stop_flag.load(Ordering::Relaxed) {
                return None;
            }

            let now = Instant::now();
            let should_process = {
                if let Ok(mut last_data) = last_event_data_clone.try_lock() {
                    let elapsed_time = now.duration_since(last_data.0);
                    let should_proc = elapsed_time > debounce_duration
                        || Some(event.event_type.clone()) != last_data.1;
                    if should_proc {
                        last_data.0 = now;
                        last_data.1 = Some(event.event_type.clone());
                    }
                    should_proc
                } else {
                    false
                }
            };

            if should_process {
                match event.event_type {
                    EventType::ButtonPress(btn) => {
                        if let Ok(coords) = coordinates_clone.try_lock() {
                            let mouse_event = MouseEvent {
                                x: coords.0,
                                y: coords.1,
                                button: Some(btn),
                            };
                            emit(mouse_event.clone());
                            if let Ok(mut queue) = event_queue.try_lock() {
                                queue.push_back(mouse_event);
                            }
                        }
                    }
                    EventType::ButtonRelease(btn) => {
                        if let Ok(coords) = coordinates_clone.try_lock() {
                            let mouse_event = MouseEvent {
                                x: coords.0,
                                y: coords.1,
                                button: Some(btn),
                            };
                            emit(mouse_event.clone());
                            if let Ok(mut queue) = event_queue.try_lock() {
                                queue.push_back(mouse_event);
                            }
                        }
                    }
                    EventType::MouseMove { x: new_x, y: new_y } => {
                        if let Ok(mut coords) = coordinates_clone.try_lock() {
                            *coords = (new_x, new_y);
                        }
                    }
                    _ => {}
                }
            }
            Some(event)
        };

        if let Err(error) = rdev::grab(func) {
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
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        if let Err(e) = rdev::exit_grab() {
            eprintln!("Error during exit_grab: {:?}", e);
        }
    }
    #[cfg(target_os = "linux")]
    {
        rdev::exit_grab_listen();
    }
    let mut handle_guard = state.thread_handle.lock().await;
    if let Some(handle) = handle_guard.take() {
        if let Err(err) = handle.join() {
            eprintln!("Error joining listener thread: {:?}", err);
        }
    }
}

/// Plays back the recorded mouse events.
pub async fn playback_events(state: MouseListenerState) -> Result<(), SimulateError> {
    let event_queue = Arc::clone(&state.event_queue);
    let last_event_played = Arc::clone(&state.last_event_played);

    let queue = event_queue.lock().await;

    if queue.is_empty() {
        println!("Event queue is empty. Nothing to playback.");
        return Ok(());
    }

    // Get the last event played
    let mut prev_event = {
        let last_event = last_event_played.lock().await;
        last_event.clone()
    };

    for event in queue.iter() {
        if let Some(prev) = prev_event {
            let steps = 10;
            let dx = (event.x - prev.x) / steps as f64;
            let dy = (event.y - prev.y) / steps as f64;

            for i in 1..=steps {
                simulate(&EventType::MouseMove {
                    x: prev.x + dx * i as f64,
                    y: prev.y + dy * i as f64,
                })?;

                sleep(Duration::from_millis(10)).await;
            }
        } else {
            simulate(&EventType::MouseMove {
                x: event.x,
                y: event.y,
            })?;
        }

        match event.button {
            Some(Button::Left) => {
                simulate(&EventType::ButtonPress(Button::Left))?;
                sleep(Duration::from_millis(50)).await;
                simulate(&EventType::ButtonRelease(Button::Left))?;
            }
            Some(Button::Right) => {
                simulate(&EventType::ButtonPress(Button::Right))?;
                sleep(Duration::from_millis(50)).await;
                simulate(&EventType::ButtonRelease(Button::Right))?;
            }
            Some(Button::Middle) => {
                simulate(&EventType::ButtonPress(Button::Middle))?;
                sleep(Duration::from_millis(50)).await;
                simulate(&EventType::ButtonRelease(Button::Middle))?;
            }
            _ => {}
        }
        // Update the last event played
        {
            let mut last_event = last_event_played.lock().await;
            *last_event = Some(event.clone());
        }

        prev_event = Some(event.clone());
    }

    Ok(())
}

pub async fn start_playback_loop(state: MouseListenerState) {
    let playback_flag = Arc::clone(&state.playback_flag);
    playback_flag.store(true, Ordering::Relaxed);

    let state_clone = state.clone();
    let handle = tokio::spawn(async move {
        println!("Starting playback...");
        while playback_flag.load(Ordering::Relaxed) {
            if let Err(e) = playback_events(state_clone.clone()).await {
                eprintln!("Error during playback_events: {:?}", e);
            }
        }
    });

    let mut handle_guard = state.playback_thread_handle.lock().await;
    *handle_guard = Some(handle);
}

pub async fn stop_playback_loop(state: MouseListenerState) {
    let playback_flag = Arc::clone(&state.playback_flag);
    playback_flag.store(false, Ordering::Relaxed);

    let mut handle_guard = state.playback_thread_handle.lock().await;
    if let Some(handle) = handle_guard.take() {
        handle.abort();
        println!("Playback stopped.");
    } else {
        println!("No playback is running.");
    }
}
