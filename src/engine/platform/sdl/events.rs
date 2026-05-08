// src/engine/platform/sdl/events.rs

use sdl3::event::Event;
use sdl3::keyboard::Keycode;

pub fn handle_events(event_pump: &mut sdl3::EventPump, running: &mut bool) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                *running = false;
            }

            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                *running = false;
            }

            _ => {}
        }
    }
}