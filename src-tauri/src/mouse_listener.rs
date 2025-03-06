use rdev::{listen, EventType};
use std::time::{Duration, Instant};

pub fn run_mouse_listener<F>(emit: F)
where
    F: Fn(&str) + 'static,
{
    let mut last_event_time = Instant::now();
    let debounce_duration = Duration::from_millis(50); // Adjust based on system behavior
    let mut last_event_type: Option<EventType> = None;

    if let Err(error) = listen(move |event| {
        let now = Instant::now();
        let elapsed_time = now.duration_since(last_event_time);

        // Check if the event type is the same as the last one and if it's within the debounce duration
        if elapsed_time > debounce_duration || Some(event.event_type.clone()) != last_event_type {
            match event.event_type {
                EventType::ButtonPress(button) => {
                    println!("{:?}", event);
                    emit(&format!("Mouse button pressed: {:?}", button));
                }
                EventType::ButtonRelease(button) => {
                    println!("{:?}", event);
                    emit(&format!("Mouse button released: {:?}", button));
                }
                _ => {}
            }

            // Update the last event type and time
            last_event_type = Some(event.event_type.clone());
            last_event_time = now;
        }
    }) {
        eprintln!("Error: {:?}", error);
    }
}
