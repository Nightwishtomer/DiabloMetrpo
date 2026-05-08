// engine/platform/winit_wgpu/window.rs

use std::sync::Arc;
use winit::window::{self, Window};

use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowAttributes;

use crate::engine::platform::winit_wgpu::event_loop;

/// Представляет окно игры на уровне platform layer.
///
/// Это abstraction поверх `winit::window::Window`.
/// 
/// Задачи:
/// - хранить ссылку на окно (Arc для безопасного шаринга)
/// - хранить базовые параметры (ширина, высота, заголовок)
/// - быть точкой входа для дальнейшего расширения (fullscreen, resize, DPI и т.д.)
///
/// ВАЖНО:
/// Gameplay и engine core НЕ должны работать напрямую с winit.
/// Всё идет через такие обертки.
pub struct GameWindow {

    /// Нативное окно winit, обернутое в Arc.
    ///
    /// Почему Arc:
    /// - окно может понадобиться в renderer, input и других системах
    /// - безопасное разделение владения между подсистемами
    pub window: Arc<Window>,

    /// Ширина окна (логическая)
    ///
    /// Сейчас фиксированная.
    /// Позже:
    /// - resize
    /// - fullscreen
    /// - config-driven значения
    pub width: u32,

    /// Высота окна (логическая)
    pub height: u32,

    /// Заголовок окна
    ///
    /// Сейчас статический.
    /// Позже можно менять:
    /// - FPS в заголовке
    /// - debug info
    pub title: String,
}
 
impl GameWindow {

    /// Создает новое окно через winit.
    ///
    /// # Параметры
    /// - `event_loop`: активный event loop от winit (обязателен для создания окна)
    ///
    /// # Что происходит внутри
    /// 1. Создаются WindowAttributes (размер, заголовок)
    /// 2. Через event_loop создается окно
    /// 3. Оборачиваем окно в Arc
    /// 4. Возвращаем GameWindow abstraction
    ///
    /// # Почему через ActiveEventLoop
    /// В winit 0.30:
    /// - окна можно создавать ТОЛЬКО внутри lifecycle (resumed)
    /// - это часть новой архитектуры event-driven runtime
    ///
    /// # Потенциальные улучшения (будущие шаги)
    /// - читать параметры из config
    /// - fullscreen / borderless
    /// - DPI scaling
    /// - multi-monitor support
    /// - vsync integration
    ///
    /// # Паника
    /// Если окно не удалось создать → panic.
    /// В будущем можно заменить на Result + логгер.
    pub fn create(
        event_loop: &ActiveEventLoop,
    ) -> Self {

        // Создаем атрибуты окна:
        // - заголовок
        // - размер (логические пиксели)
        let attributes = WindowAttributes::default()
            .with_title("DiabloMetro")
            .with_inner_size(LogicalSize::new(800,600));

        // Создаем окно через winit API
        let window = event_loop
            .create_window(attributes)
            .expect("Failed to create window");

        // Оборачиваем в нашу abstraction
        Self {
            
            // ВАЖНО:
            // Сейчас значения захардкожены.
            // Позже они должны синхронизироваться с реальным окном.
            window: Arc::new(window),
            width: 800,
            height: 600,
            title: String::from("DiabloMetro"),
        }
    }
}


// Старый вариант через Window::default_attributes()
// Оставлен как reference для сравнения API.
//
// let window_attributes = Window::default_attributes()
//     .with_title("DiabloMetro")
//     .with_inner_size(winit::dpi::LogicalSize::new(
//         800.0, 600.0
//         //1280.0, 720.0
//     ));