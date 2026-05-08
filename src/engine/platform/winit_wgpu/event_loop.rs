// engine/platform/winit_wgpu/event_loop.rs

//! Platform layer: Winit event loop integration.
//!
//! Этот файл — мост между OS (winit) и нашим Engine.
//! ВАЖНО:
//! - здесь НЕТ игровой логики
//! - здесь НЕТ gameplay
//! - здесь только:
//!     → обработка событий ОС
//!     → прокидывание их в Engine
//!
//! Всё остальное делает engine.process_frame()

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;

use winit::event::ElementState;
use winit::keyboard::{KeyCode, PhysicalKey};

use crate::engine::core::app_state::AppState;
use crate::engine::core::application::Application;
use crate::engine::core::engine::Engine;

/// Runtime-обёртка над Engine для интеграции с winit.
///
/// Почему это отдельно:
/// - winit требует свой trait (ApplicationHandler)
/// - engine НЕ должен зависеть от конкретной платформы
///
/// Поэтому:
/// EngineRuntime = адаптер (adapter pattern)
pub struct EngineRuntime<T: Application> {

    /// Основной движок
    pub engine: Engine<T>,
}

impl<T: Application> ApplicationHandler for EngineRuntime<T> {

    /// Вызывается когда приложение "оживает" (resumed).
    ///
    /// Это происходит:
    /// - при старте приложения
    /// - при возвращении из suspend (например alt-tab / mobile resume)
    ///
    /// Что мы делаем:
    /// 1. Инициализируем окно (platform layer)
    /// 2. Запрашиваем первый redraw (запуск цикла кадров)
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        // Инициализация окна через platform abstraction
        self.engine
            .platform
            .initialize_window(event_loop);

        // Запускаем первый кадр (без этого не начнётся render loop)
        if let Some(window) = &self.engine.platform.window {
            window.window.request_redraw();
        }
    }

    /// Главная точка входа всех событий окна и ввода.
    ///
    /// Это единственное место, где мы:
    /// - получаем события от ОС
    /// - решаем что с ними делать
    ///
    /// ВАЖНО:
    /// - никакой gameplay логики здесь быть не должно
    /// - только перевод событий → вызовы Engine
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) { 
        match event {

            /// Событие закрытия окна (крестик).
            ///
            /// Мы НЕ выходим сразу!
            /// Вместо этого:
            /// → отправляем запрос на shutdown
            ///
            /// Почему:
            /// Engine должен корректно завершиться (cleanup ресурсов)
            WindowEvent::CloseRequested => {
                //println!("Close requested");
                self.engine
                    .logger
                    .info("Window close requested");
                // self.engine.running = false;
                // self.engine.state = AppState::Shutdown;


                // self.engine.shutdown();    
                // event_loop.exit();
                self.engine.request_shutdown();
            }

            /// Событие перерисовки окна.
            ///
            /// Это сердце runtime:
            /// → здесь вызывается engine.process_frame()
            ///
            /// Что происходит внутри:
            /// - time update
            /// - fixed update
            /// - update
            /// - render
            /// - frame limiting
            WindowEvent::RedrawRequested => {

                // Один полный кадр движка
                self.engine.process_frame();

                // Если после кадра мы в состоянии Shutdown → выходим из event loop
                if self.engine.state == AppState::Shutdown {
                    event_loop.exit();
                    return;
                }
                // let frame_start = std::time::Instant::now();
                // self.engine.time.update();

                // let dt = self.engine.time.delta;
                // self.engine.time.accumulator += dt;

                // while  self.engine.time.accumulator >= self.engine.time.fixed_delta {
                //     self.engine.application.fixed_update();

                    

                //     self.engine.time.accumulator -= self.engine.time.fixed_delta;
                    
                // }

                // self.engine
                //     .fps_counter
                //     .update(dt);

                // self.engine.application.update();

                // self.engine.application.render();

                // self.engine
                //     .time
                //     .limit_frame_rate(frame_start);

                // Запрашиваем следующий кадр (continuous loop)
                if let Some(window) = 
                    &self.engine.platform.window {
                    window.window.request_redraw();
                }

                // debug (пока)
              //  println!("dt: {:.4}", dt);
            }


            /*
            
            ENTER  → начать игру (из меню)
            ESC    → выйти из игры (shutdown)
            P      → пауза / продолжить
            SPACE  → оставим под gameplay (потом)

            MainMenu --(ENTER)--> InGame
            InGame --(P)--> Paused
            Paused --(P)--> InGame
            ESC → Shutdown (из любого состояния)

            */

            /// Обработка клавиатуры (raw input от winit).
            ///
            /// ВАЖНО:
            /// Это временное решение.
            ///
            /// Позже будет:
            /// engine/input/*
            /// → InputManager
            /// → keybindings
            /// → actions
            WindowEvent::KeyboardInput { event, .. } => {

                 // Нас интересуют только нажатия (не отпускание)
                if event.state == ElementState::Pressed {
                
                    /// ESC → запрос на shutdown
                    if let PhysicalKey::Code(KeyCode::Escape) =
                        event.physical_key
                    {
                        println!("ESC pressed");

                        // self.engine.running = false;
                        // self.engine.state = AppState::Shutdown;
                        
                        // self.engine.shutdown();



                        
                        // НЕ выходим сразу!
                        // Только ставим флаг
                        self.engine.request_shutdown();
                    }
               
                    /// ENTER → вход в игру (из меню)
                    if let PhysicalKey::Code(KeyCode::Enter) = event.physical_key {
                        if self.engine.state == AppState::MainMenu {
                            self.engine.enter_game();
                            println!("Enter Game");
                        }
                        
                    }

                    /// SPACE → пока зарезервирован под gameplay
                    if let PhysicalKey::Code(KeyCode::Space) = event.physical_key {
                        // self.engine.enter_game();

                        println!("Space pressed (reserved for gameplay)");
                    }

                
                    /// P → переключение паузы
                    ///
                    /// Работает только:
                    /// InGame ↔ Paused
                    if let PhysicalKey::Code(KeyCode::KeyP) = event.physical_key {
                        println!("Key P");
                        match self.engine.state {
                            AppState::InGame => {
                                self.engine.pause();
                                println!("Paused");
                            }

                            AppState::Paused => {
                                self.engine.resume();
                                println!("Resumed");
                            }

                            /// Все остальные события игнорируем
                            _ => {}
                        }
                        
                    }                   
                
                

                }

                


            }   

            _ => {}
        }
    }
}