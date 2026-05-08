// src/game/game.rs

use crate::engine::core::application::Application;

/// Главная структура игры.
/// 
/// Здесь позже будут:
/// - ссылки на ECS World
/// - ресурсы (textures, audio, config)
/// - состояние игрока
/// - менеджеры (UI, сцены, и т.д.)
pub struct Game;

/// Реализация Application — это точка входа gameplay-логики.
/// 
/// Engine вызывает эти методы в строгом порядке:
/// init → update/fixed_update → render → shutdown
impl Application for Game {

    /// Вызывается ОДИН раз при старте приложения.
    /// 
    /// Здесь:
    /// - инициализация ресурсов
    /// - создание мира (entities, ECS)
    /// - загрузка текстур / звуков
    /// - настройка начального состояния
    fn init(&mut self) {
        println!("Game init");
    }

    /// Variable update (каждый кадр).
    /// 
    /// Частота вызова зависит от FPS.
    /// 
    /// Здесь:
    /// - input обработка (пока сырая)
    /// - камера
    /// - UI логика
    /// - визуальные эффекты
    /// 
    /// ❗ НЕ ДЕЛАТЬ:
    /// - физику
    /// - важную gameplay-логику (она должна быть в fixed_update)
    fn update(&mut self) {
        // println!("Game update");
    }
    
    /// Fixed update (фиксированный timestep).
    /// 
    /// Вызывается стабильно (например 60 раз в секунду),
    /// независимо от FPS.
    /// 
    /// Здесь:
    /// - движение
    /// - физика
    /// - столкновения
    /// - AI
    /// - combat
    /// 
    /// ❗ Это САМОЕ ВАЖНОЕ место для gameplay.
    fn fixed_update(&mut self) {
        // println!("Game update");
    }

    /// Update во время паузы.
    /// 
    /// Вызывается ТОЛЬКО когда AppState = Paused.
    /// 
    /// Здесь:
    /// - UI меню (пауза)
    /// - кнопки (resume, exit)
    /// - курсор
    /// - анимации интерфейса
    /// 
    /// ❗ Gameplay здесь НЕ должен обновляться
    fn update_paused(&mut self) {
        // println!("Paused update (UI, menu, etc)");
    }

    /// Рендер кадра.
    /// 
    /// Вызывается каждый кадр.
    /// 
    /// Здесь:
    /// - отрисовка мира
    /// - спрайты
    /// - UI
    /// - debug overlay
    /// 
    /// ❗ НЕ делать:
    /// - изменение логики
    /// - изменение состояния мира
    fn render(&mut self) {
        // println!("Game render");
        // renderer.draw_sprite(...)
        // renderer.draw_rect(...)
    }

    /// Вызывается при завершении приложения.
    /// 
    /// Здесь:
    /// - сохранение игры
    /// - освобождение ресурсов
    /// - логирование
    /// 
    /// ❗ Важно:
    /// вызывается через shutdown pipeline (Шаг 18)
    fn shutdown(&mut self) {
        println!("Game shutdown");
    }


    // /// УСТАРЕВШЕЕ / НЕ НУЖНО
    // /// 
    // /// fixed timestep теперь управляется Engine,
    // /// а не через отдельную функцию.
    // fn fixed_delta(&mut self) {
    //     println!("Fixed update");
    // }
} 

