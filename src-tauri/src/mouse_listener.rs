use std::time::{Duration, Instant};

use rdev::{listen, EventType};

pub fn run_mouse_listener<F>(emit: F)
where
    F: Fn(&str) + 'static,
{
    let mut last_event_time = Instant::now();
    let debounce_duration = Duration::from_millis(50); // Adjust based on system behavior
    let mut last_event_type: Option<EventType> = None;
    if let Err(error) = listen(move |event| match event.event_type {
        EventType::ButtonPress(button) => {
            println!("{:?}", event);
            emit(&format!("Mouse button pressed: {:?}", button));
        }
        EventType::ButtonRelease(button) => {
            println!("{:?}", event);
            emit(&format!("Mouse button released: {:?}", button));
        }
        _ => {}
    }) {
        eprintln!("Error: {:?}", error);
    }
}
