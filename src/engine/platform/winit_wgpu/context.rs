// engine/platform/winit_wgpu/context.rs

// use winit::window;


// Контекст платформы (Platform Layer abstraction)
//
// Это слой, который изолирует движок от конкретной библиотеки окон/OS (в нашем случае winit).
// В будущем здесь может появиться:
// - input system binding
// - audio context
// - GPU context (wgpu / Vulkan / OpenGL)
// - platform-specific API
//
// ВАЖНО:
// Engine НЕ должен знать про winit напрямую.
// Он работает только с PlatformContext.
use winit::event_loop::{self, ActiveEventLoop};
 
use super::window::GameWindow;

/// PlatformContext
///
/// Главная точка входа в platform layer.
///
/// Хранит:
/// - окно (GameWindow)
/// - в будущем: input, audio, renderer context и т.д.
///
/// Почему Option:
/// Окно НЕ может быть создано сразу при старте Engine,
/// потому что winit требует создавать его внутри event loop (resumed()).
///
/// Поэтому:
/// - сначала Engine создается
/// - потом winit запускает lifecycle
/// - и только тогда мы создаем окно
pub struct PlatformContext {

    /// Окно приложения
    ///
    /// Option используется, потому что:
    /// - окно создается НЕ сразу
    /// - зависит от lifecycle winit
    pub window: Option<GameWindow>,
}

impl PlatformContext {

    /// Создает пустой PlatformContext
    ///
    /// На этом этапе:
    /// - окно еще НЕ создано
    /// - мы только подготавливаем структуру
    ///
    /// Это нормально, потому что:
    /// winit требует создавать окно внутри event loop.
    pub fn new() -> Self {
        Self {
            // window: GameWindow {
            //     width: 800,
            //     height: 600,
            //     title: String::from("DiabloMetro"),
            // },

            // Раньше здесь мог быть статический window config,
            // но теперь окно создается динамически через winit lifecycle.
            window: None,
        }
        
    }

    /// Инициализация окна через winit lifecycle
    ///
    /// Вызывается из:
    /// - event_loop.rs → ApplicationHandler::resumed()
    ///
    /// Почему именно здесь:
    /// winit НЕ позволяет создавать окно вне ActiveEventLoop.
    ///
    /// Что происходит:
    /// 1. Получаем ActiveEventLoop от winit
    /// 2. Создаем GameWindow (наша абстракция)
    /// 3. Сохраняем его в PlatformContext
    ///
    /// ВАЖНО:
    /// Это граница между:
    /// - platform layer (winit)
    /// - engine abstraction (GameWindow)
    pub fn initialize_window(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) {
        // Создаем окно через нашу абстракцию,
        // чтобы engine не зависел напрямую от winit::window
        self.window = Some(GameWindow::create(event_loop));

    }
}