// src/engine/core/app_state.rs

/// Represents the current high-level state of the application.
///
/// This enum is the core of the engine's runtime state machine.
/// It controls which parts of the frame pipeline are executed:
/// - gameplay (simulation)
/// - UI updates
/// - rendering
/// - shutdown sequence
///
/// IMPORTANT:
/// AppState does NOT represent scenes (yet).
/// It represents the global lifecycle of the application.
///
/// In the future this will work together with:
/// - SceneManager
/// - UI system
/// - Loading pipeline
///
/// The engine behavior is fully driven by this state.
#[derive(Debug, PartialEq, Eq)]
pub enum AppState {
    //Running,

    /// Main menu state.
    ///
    /// Active systems:
    /// - UI (menus, buttons)
    /// - input (menu navigation)
    ///
    /// Disabled:
    /// - gameplay simulation
    /// - physics
    /// - AI
    ///
    /// Notes:
    /// This is the default startup state of the engine.
    MainMenu,

    /// Loading state.
    ///
    /// Active systems:
    /// - resource loading
    /// - map generation
    /// - initialization logic
    ///
    /// Disabled:
    /// - gameplay
    /// - input (partially or fully)
    ///
    /// Notes:
    /// Usually transient. Should transition into `InGame`.
    Loading,

    /// Main gameplay state.
    ///
    /// Active systems:
    /// - fixed_update (physics, movement, AI)
    /// - update (game logic)
    /// - rendering
    ///
    /// This is the "full pipeline" state.
    ///
    /// Notes:
    /// Most of the engine runtime operates in this state.
    InGame,

    /// Pause state.
    ///
    /// Active systems:
    /// - UI updates (pause menu, overlays)
    /// - rendering
    ///
    /// Disabled:
    /// - fixed_update (physics)
    /// - gameplay update
    ///
    /// Notes:
    /// Simulation is frozen, but presentation layer continues.
    Paused,

    /// Shutdown state.
    ///
    /// Active systems:
    /// - shutdown pipeline (resource cleanup)
    ///
    /// Disabled:
    /// - all runtime updates
    ///
    /// Notes:
    /// This state is terminal. After entering this state,
    /// the engine will exit the main loop.
    Shutdown,
}
/*
 * MainMenu
 * - UI
 * - кнопки
 * - нет gameplay
 * 
 * Loading
 * - загрузка ресурсов
 * - генерация карты
 * - блок gameplay
 * 
 * InGame
 * - основной runtime
 * - gameplay
 * - AI
 * - physics
 * 
 * Paused
 * - gameplay frozen
 * - UI работает
 * 
 * Shutdown
 * - завершение
*/

/*
 * Потому что у движка есть состояния:
 * - игра идет
 * - игра на паузе
 * - загрузка уровня
 * - shutdown
 * - main menu
 * - cinematic
 * - editor mode
 */ 