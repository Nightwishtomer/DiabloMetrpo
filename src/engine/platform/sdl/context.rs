// src/engine/platform/sdl/context.rs

use sdl3::init;
use sdl3::sys::video;
use sdl3::video::Window;

pub struct SdlContext {
    pub running: bool,
    pub sdl: sdl3::Sdl,
    pub video: sdl3::VideoSubsystem,
}

impl SdlContext {
    pub fn new() -> Self {
        let sdl = init().expect("Failed to init SDL3");
        let video = sdl.video().expect("Failed to init video subsystem");
        println!("SDL init initialized");
        Self {
            running: true,
            sdl,
            video,
        }
    }

    pub fn poll_events(&mut self, running: &mut bool) {
        // пока заглушка

        // позже тут будет SDL event loop
        // сейчас просто симуляция закрытия окна

        *running = true;
    }

}