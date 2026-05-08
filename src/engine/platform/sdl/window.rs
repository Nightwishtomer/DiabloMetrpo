// src/engine/platform/sdl/window.rs

use sdl3::video::Window;

pub struct GameWindow {
    pub window: Window,
}

impl GameWindow {
    pub fn new(video: &sdl3::VideoSubsystem) -> Self {
        let window = video
            .window("Metro Diablo", 1280, 720)
            .position_centered()
            .build()
            .expect("Failed to create window");
        Self {
            window,
        }
    }
}