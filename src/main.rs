// src/main.rs

// Подключаем модуль движка (engine)
// Здесь лежит весь runtime: loop, platform, renderer, ECS и т.д.
mod engine;

// Подключаем игровой модуль (game)
// Здесь будет gameplay-логика, системы, сцены и т.п.
mod game;

// Импортируем основной Engine (ядро движка)
use engine::core::engine::Engine;

// Импортируем Game — реализацию Application trait
use game::game::Game;

/// Точка входа приложения.
///
/// Здесь происходит bootstrap всего движка:
/// 1. Создается игровая логика (Game)
/// 2. Инициализируется движок (Engine)
/// 3. Запускается runtime (event loop)
///
/// Важно:
/// - main() НЕ содержит игровой логики
/// - main() только связывает engine + game
/// - вся архитектура дальше живет внутри Engine
fn main() {
    println!("Metro Diablo Engine Bootstrap");
    
    // Создаем экземпляр игры.
    //
    // Game реализует trait Application, который определяет:
    // - init()
    // - update()
    // - fixed_update()
    // - render()
    // - shutdown()
    //
    // Важно:
    // здесь пока нет состояния — это просто "объект логики".
    let mut game = Game;

    // Создаем Engine и передаем в него игру.
    //
    // Engine:
    // - берет ownership Game
    // - управляет lifecycle
    // - запускает event loop
    // - вызывает методы Application
    //
    // После этого Game больше не управляется напрямую.
    let mut engine = Engine::new(game);

    // Запуск движка.
    //
    // Внутри происходит:
    // - создание event loop (winit)
    // - инициализация платформы (window, input)
    // - запуск runtime pipeline:
    //     process_frame()
    //         ├── time.update()
    //         ├── fixed_update()
    //         ├── update()
    //         ├── render()
    //         └── frame limiter
    //
    // Важно:
    // эта функция блокирует поток до завершения приложения.
    engine.run();

}