// src/engine/renderer/renderer.rs

pub trait Renderer {

    /// подготовка кадра
    /// сброс state
    fn begin_frame(&mut self);

    /// завершение команд
    fn end_frame(&mut self);

    /// очистка экрана
    fn clear(&mut self);

    /// вывод на экран
    fn present(&mut self);

}


pub struct DummyRenderer;

impl Renderer for DummyRenderer {
    fn begin_frame(&mut self) {
        // println!("Begin frame");
    }

    fn end_frame(&mut self) {
        // println!("End frame");
    }

    fn clear(&mut self) {
        // println!("Clear screen");
    }

    fn present(&mut self) {
        // println!("Present frame");
    }
}