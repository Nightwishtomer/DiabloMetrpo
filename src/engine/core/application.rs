// src/engine/core/application.rs

/// Главный интерфейс игры (Game Layer).
/// 
/// Engine вызывает эти методы в рамках своего runtime pipeline.
/// 
/// ВАЖНО:
/// Application НЕ управляет временем, loop или платформой.
/// Он только реагирует на вызовы Engine.
///
/// Архитектурно:
/// Engine → вызывает → Application
pub trait Application {
    
    ///
    /// init()
    ///
    /// Вызывается один раз при запуске приложения.
    ///
    /// Используется для:
    /// - загрузки ресурсов (textures, sounds)
    /// - инициализации ECS / world
    /// - создания начального состояния игры
    /// - настройки систем
    ///
    /// ВАЖНО:
    /// - НЕ делать тяжелые блокирующие операции (в будущем будет Loading state)
    /// - НЕ создавать runtime loop
    ///
    fn init(&mut self);

 
    ///
    /// update()
    /// 
    /// Variable update (frame-dependent).
    /// 
    /// Вызывается:
    /// - каждый frame
    /// - зависит от FPS
    /// 
    /// Используется для:
    /// - визуальных обновлений
    /// - UI логики
    /// - camera smoothing
    /// - interpolation (между fixed шагами)
    /// - debug систем
    /// 
    /// ВАЖНО:
    /// - НЕ использовать для gameplay логики
    /// - НЕ использовать для physics
    /// 
    /// Почему:
    /// FPS нестабилен → логика будет ломаться
    /// 
    /// ⚠ В будущем может быть:
    /// - частично заменен
    /// или разделен на несколько stages
    ///
    fn update(&mut self); // удалить. у него плавает скорость


    ///
    /// fixed_update()
    ///
    /// Fixed timestep update (deterministic).
    ///
    /// Вызывается:
    /// - фиксированное количество раз в секунду (например 60)
    /// - независимо от FPS
    /// 
    ///  Используется для:
    /// - gameplay логики
    /// - physics
    /// - collision
    /// - movement
    /// - combat
    /// - AI
    /// - ECS систем
    /// 
    /// ВАЖНО:
    /// - ВСЯ игровая логика должна жить здесь
    /// - гарантирует одинаковое поведение на всех машинах
    /// 
    /// Это:
    /// ✔ основа deterministic simulation
    /// ✔ база для multiplayer
    ///
    /// Fixed update (deterministic)
    fn fixed_update(&mut self);

    //fn fixed_delta(&mut self);
 

    /**
     * update_paused()
     *
     * Вызывается только когда приложение находится в состоянии Paused.
     *
     * Используется для:
     * - UI (меню паузы)
     * - курсор
     * - кнопки
     * - overlay
     * - debug интерфейс
     *
     * НЕ вызывается:
     * - fixed_update()
     * - gameplay update
     *
     * ВАЖНО:
     * - здесь НЕ должно быть gameplay логики
     * - мир игры "заморожен"
     *
     * Архитектурно:
     * Simulation остановлена
     * Presentation продолжает работать
     */
    /// Fixed deterministic update
    fn update_paused(&mut self);


     /**
     * render()
     *
     * Отвечает ТОЛЬКО за отрисовку.
     *
     * Вызывается каждый frame (даже в Paused).
     *
     * Используется для:
     * - отправки draw команд в renderer
     * - отрисовки спрайтов
     * - UI
     * - debug визуализации
     *
     * ВАЖНО:
     * - НЕ изменять состояние игры
     * - НЕ делать логику
     *
     * Позже:
     * будет заменен на renderer API (command queue)
     */
    /// Rendering only
    fn render(&mut self);

    /**
     * shutdown()
     *
     * Вызывается один раз при завершении приложения.
     *
     * Используется для:
     * - сохранения игры
     * - освобождения ресурсов
     * - закрытия файлов
     * - остановки аудио
     *
     * ВАЖНО:
     * - вызывается из Engine shutdown pipeline
     * - гарантируется, что будет вызван перед exit
     */
    fn shutdown(&mut self);
    //fn request_shutdown(&mut self);

    

}