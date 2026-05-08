// src/engine/core/engine.rs

// use winit::application;

// Главный контракт приложения (игры)
// Engine не знает, что именно за игра — только вызывает API
use crate::engine::core::application::Application;

// Платформенный слой (окно, контекст, интеграция с winit/wgpu)
use crate::engine::platform::winit_wgpu::context::PlatformContext;

// Event loop из winit — управляет жизненным циклом приложения
use winit::event_loop::EventLoop;

// Runtime-обёртка, которая связывает Engine и event loop
use crate::engine::platform::winit_wgpu::event_loop::EngineRuntime;

// Система времени (delta, accumulator, fixed timestep)
use crate::engine::core::time::Time;

// Состояние приложения (MainMenu, InGame и т.д.)
use crate::engine::core:: app_state::AppState;

// FPS счётчик (debug инструмент)
use crate::engine::debug::fps_counter::FPSCounter;

// FPS счётчик (debug инструмент)
use crate::engine::core::logger::Logger;


use crate::engine::core::config::EngineConfig;

use crate::engine::renderer::renderer::Renderer;
use crate::engine::renderer::renderer::DummyRenderer;

/// Главная структура движка.
/// Отвечает за:
/// - управление жизненным циклом
/// - execution pipeline
/// - состояние приложения
/// - связь с платформой
pub struct Engine<T: Application> {
    //pub running: bool,
    pub application: T,
    pub state: AppState,
    pub platform: PlatformContext,
    pub time: Time,
    pub fps_counter: FPSCounter,
    pub shutdown_requested: bool,
    pub logger: Logger,
    pub config: EngineConfig,
    pub renderer: Box<dyn Renderer>,
    
}

impl<T: Application> Engine<T> {

    /// Создает новый Engine.
    ///
    /// Здесь происходит:
    /// - инициализация всех подсистем
    /// - установка начального состояния (MainMenu)
    ///
    /// ВАЖНО:
    /// Engine ещё НЕ запущен, только подготовлен.
    pub fn new(application: T) -> Self {
        

        let config = EngineConfig::load("config/engine.ron");

        // 1. создаем time отдельно
        let mut time = Time::new();

        // 2. применяем config
        let mut time = Time::new();
        time.apply_config(&config);

        // 3. создаем logger
        let mut logger = Logger::new();

        // 4. логируем config
        logger.info(&format!("Config loaded: {:?}", config));
        
        Self {
            // Конкретная реализация игры
            application, // pub

            // Текущее состояние приложения (state machine)
            state: AppState::MainMenu,// pub

            // Платформенный контекст (окно, surface, input)
            platform: PlatformContext::new(),// pub

            // Система времени
            //time: Time::new(),// pub
            time,

            // FPS debug инструмент
            fps_counter: FPSCounter::new(),// pub

            // Флаг: запрошено ли завершение приложения
            // ВАЖНО: это НЕ мгновенный выход
            shutdown_requested: false,// pub

            // Глобальный логгер движка
            logger: Logger::new(),// pub

            config,

            renderer: Box::new(DummyRenderer),
        }

        

    }


    /// Запускает основной цикл приложения.
    ///
    /// Делает:
    /// 1. Создает event loop (winit)
    /// 2. Оборачивает Engine в runtime
    /// 3. Вызывает init() у игры
    /// 4. Передает управление winit
    /// 5. После выхода вызывает shutdown()
    ///
    /// ВАЖНО:
    /// После run() управление НЕ возвращается,
    /// пока приложение не завершится.
    pub fn run(self) {
        
        let event_loop =
            EventLoop::new().expect("Failed to create event loop");

        let mut runtime = EngineRuntime {
            engine: self,
        };

        runtime.engine.application.init();

        event_loop
            .run_app(&mut runtime)
            .expect("Failed to run app");

        runtime.engine.application.shutdown();


   

    }


    /// Обрабатывает ОДИН кадр.
    ///
    /// Это сердце движка.
    ///
    /// Pipeline:
    /// - проверка shutdown
    /// - update времени
    /// - выбор логики по AppState
    /// - выполнение стадий (fixed/update/render)
    /// - ограничение FPS
    pub fn process_frame(&mut self){
        if self.shutdown_requested {
            self.perform_shutdown();
            return;
        }
        let frame_start = std::time::Instant::now();

        self.time.update();
        let dt = self.time.delta;

        match self.state {
      
            AppState::MainMenu => {
                self.update_stage(dt);
                self.render_stage();
               // return;
            }

            AppState::Loading => {
                // пока просто рендер
                self.render_stage();
              //  return;
            }

            AppState::InGame => {
                // полный pipeline
                self.time.accumulator += dt;

                self.fixed_update_stage();
                
                self.update_stage(dt);

                self.render_stage();

                //return;
            }

            AppState::Paused => {
                // self.application.render();
                self.update_paused_stage(dt);
                self.render_stage();
              //  return;
            }

            AppState::Shutdown => {
                return;
            }
            
        }

        



        self.time.limit_frame_rate(frame_start);
            
    }

    /// Fixed Update Stage.
    ///
    /// Выполняется с фиксированным шагом времени.
    ///
    /// Здесь будет:
    /// - physics
    /// - collision
    /// - AI
    /// - ECS fixed systems
    ///
    /// Может вызываться несколько раз за кадр.
    pub fn fixed_update_stage(&mut self) {
        while self.time.accumulator >=self.time.fixed_delta {
            self.application.fixed_update();

            self.time.accumulator -= self.time.fixed_delta;
        }
    }
        
    /// Variable Update Stage.
    ///
    /// Выполняется 1 раз за кадр.
    ///
    /// Используется для:
    /// - gameplay логики
    /// - анимаций
    /// - UI логики
    pub fn update_stage(&mut self, dt: f32) {
        self.fps_counter.update(dt);
        self.application.update();
    }

    /// Render Stage.
    ///
    /// Отвечает ТОЛЬКО за отрисовку.
    ///
    /// В будущем:
    /// - renderer.begin_frame()
    /// - draw calls
    /// - renderer.end_frame()
    pub fn render_stage(&mut self) {
        // self.application.render();

        self.renderer.begin_frame();

        self.renderer.clear();

        self.application.render();

        self.renderer.end_frame();

        self.renderer.present();
    }

    // =========================
    // STATE MANAGEMENT
    // =========================

    /// Переход в игровой режим
    pub fn enter_game(&mut self) {
        self.state = AppState::InGame;
    }

    // Переход в меню
    pub fn enter_menu(&mut self) {
        self.state = AppState::MainMenu;
    }

    // Переход в состояние загрузки
    pub fn start_loading(&mut self) {
        self.state = AppState::Loading;
    }

    /// Ставит игру на паузу (только из InGame)
    pub fn pause(&mut self) {
        if self.state == AppState::InGame {
            self.state = AppState::Paused;
        }
    }

    /// Возвращает игру из паузы
    pub fn resume(&mut self) {
        if self.state == AppState::Paused {
            self.state = AppState::InGame;
        }
    }

    /// Update при паузе.
    ///
    /// Здесь будет:
    /// - UI
    /// - курсор
    /// - меню
    /// - эффекты
    pub fn update_paused_stage(&mut self, dt: f32) {
        self.application.update_paused();

        /*
         * Пока dt не используем
         * позже:
         * - Анимация UI
         * - Курсор
         * - hover эффекты
         * - tweening
         */
    }

    /// Запрос на завершение приложения.
    ///
    /// ВАЖНО:
    /// Это НЕ мгновенный выход.
    /// Engine сам завершит работу через pipeline.
    pub fn request_shutdown(&mut self) {
        self.shutdown_requested = true;
    }

    /// Выполняет корректное завершение движка.
    ///
    /// Делает:
    /// - shutdown игры
    /// - (в будущем) очистку ресурсов
    /// - перевод состояния в Shutdown
    ///
    /// Вызывается ТОЛЬКО внутри process_frame.
    fn perform_shutdown(&mut self) {
        if self.state == AppState::Shutdown {
            return;
        }
        // println!("Engine shutdown started");
        self.logger.info("Engine shutdown started");
        self.application.shutdown();
//Нужно поросить chatgpt сделать комментарии ко всем функциям во всех файлах. вне зависимости: будут ли они потом удаляться или нет
        // позже:
        // audio.shutdown()
        // renderer.shutdown()
        // ecs.cleanup()
        // resources.clear()

        // println!("Engine shutdown completed");
        self.logger.info("Engine shutdown completed");

        self.state = AppState::Shutdown;
    }   

} 

