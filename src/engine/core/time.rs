// src/engine/core/time.rs

use std::time::Duration;
use std::time::Instant;
use std::thread;


use crate::engine::core::config::EngineConfig;

/// Time — центральная система времени движка.
///
/// Отвечает за:
/// - delta time (время между кадрами)
/// - общее время работы приложения
/// - fixed timestep (для стабильной логики/физики)
/// - frame limiting (ограничение FPS)
///
/// Это один из core компонентов runtime pipeline.
pub struct Time {
    /// Время запуска приложения
    pub start: Instant,

    /// Время предыдущего кадра
    pub last_frame: Instant,

    /// Delta time (в секундах)
    /// Сколько времени прошло между текущим и прошлым кадром
    pub delta: f32,
    
    /// Общее время работы приложения (в секундах)
    pub total: f32,
    
    /// Целевой FPS (например 60)
    pub target_fps: u32,

    /// Сколько должен длиться один кадр
    /// (например ~16.6ms для 60 FPS)
    pub target_frame_time: Duration,

    /// Фиксированный timestep для deterministic update
    /// Используется в fixed_update (физика, AI и т.д.)
    pub fixed_delta: f32,

    /// Накопитель времени для fixed timestep
    ///
    /// Позволяет выполнять несколько fixed_update за кадр,
    /// если накопилось достаточно времени.
    pub accumulator: f32,
}

impl Time {

    /// Создает новую Time систему.
    ///
    /// Инициализирует:
    /// - стартовое время
    /// - значения delta/total
    /// - target FPS (по умолчанию 60)
    /// - fixed timestep
    ///
    /// Важно:
    /// target_frame_time и fixed_delta НЕ обязаны совпадать.
    /// В будущем их можно будет настраивать отдельно.
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            start: now,
            last_frame: now,
            delta: 0.0,
            total: 0.0,
            target_fps: 60,

            // 1 / FPS = длительность кадра
            target_frame_time: Duration::from_secs_f32(1.0 / 60.0),

            // фиксированный timestep (логика/физика)
            fixed_delta: 1.0 / 60.0,

            accumulator: 0.0,
        }

        
    }

    /// Обновляет время кадра.
    ///
    /// Вызывается один раз за frame.
    ///
    /// Делает:
    /// - вычисляет delta time (dt)
    /// - обновляет общее время
    /// - сдвигает last_frame
    ///
    /// Важно:
    /// delta используется в variable update,
    /// accumulator — в fixed update.
    pub fn update(&mut self) {
        let now = Instant::now();

        // Время между текущим и предыдущим кадром
        let dt = now - self.last_frame;

        // Переводим в секунды (f32 удобно для математики)
        self.delta = dt.as_secs_f32();

        // Общее время с момента старта
        self.total = (now - self.start).as_secs_f32();

        // Обновляем точку отсчета
        self.last_frame = now;
    }

    /// Ограничивает FPS до target_fps.
    ///
    /// Работает так:
    /// 1. Смотрим, сколько занял текущий кадр (update + render)
    /// 2. Если кадр слишком быстрый — "усыпляем" поток
    ///
    /// Это делает runtime:
    /// - стабильным
    /// - предсказуемым
    /// - менее прожорливым по CPU
    ///
    /// Важно:
    /// thread::sleep НЕ точный.
    /// В будущем можно заменить на hybrid limiter (sleep + spin).
    pub fn limit_frame_rate(&self, frame_start: Instant) {
        let frame_time = frame_start.elapsed();
        
        // Если кадр выполнился быстрее, чем target
        if frame_time < self.target_frame_time {
            let sleep_duration = self.target_frame_time - frame_time;

            // Усыпляем поток, чтобы выдержать нужный FPS
            thread::sleep(sleep_duration);
        }

    }




     pub fn apply_config(&mut self, config: &EngineConfig) {
        self.target_fps = config.target_fps;
        self.target_frame_time =
            std::time::Duration::from_secs_f32(
                1.0 / config.target_fps as f32,
            );
    }

}
