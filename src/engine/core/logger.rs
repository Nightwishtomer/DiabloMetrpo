// src/engine/core/logger.rs

/// Уровень логирования.
/// 
/// Используется для категоризации сообщений:
/// - Info  — обычные события (запуск, действия игрока)
/// - Warn  — подозрительные ситуации (но не критичные)
/// - Error — ошибки
/// - Debug — отладочная информация (шумная, но полезная)
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

use std::fs::{File, OpenOptions};
use std::io::Write;
use chrono::Local;

/// Основной логгер движка.
/// 
/// Отвечает за:
/// - вывод логов в консоль
/// - запись логов в файл
/// 
/// В будущем сюда можно добавить:
/// - уровни фильтрации
/// - async логирование
/// - разные output targets (файл, сеть, UI)
pub struct Logger {
    /// Файл, в который пишутся логи
    file: File,
}

impl Logger {

    /// Создает новый Logger.
    ///
    /// Что делает:
    /// 1. Генерирует имя файла (с датой и временем)
    /// 2. Открывает файл в режиме append (добавление)
    /// 3. Создает файл, если его нет
    ///
    /// Паника:
    /// - если файл не удалось открыть
    pub fn new() -> Self {
        let filename = generate_log_filename();

        let file = OpenOptions::new()
            .create(true)// создать файл если нет
            .append(true)// писать в конец
            .open(&filename)
            .expect("Failed to open log file");

        Self { file }
    }

     /// Основной метод логирования.
    ///
    /// Принимает:
    /// - уровень логирования
    /// - текст сообщения
    ///
    /// Что делает:
    /// 1. Генерирует строку времени
    /// 2. Форматирует сообщение
    /// 3. Пишет в консоль
    /// 4. Пишет в файл
    ///
    /// Формат:
    /// [HH:MM:SS][LEVEL] message
    ///
    /// Пример:
    /// [12:45:03][Info] Engine started
    pub fn log(&mut self, level: LogLevel, message: &str) {
        let timestring = current_time_string();

        let formatted = format!(
            "[{}][{:?}] {}\n",
            timestring, level, message
        );

        // Вывод в консоль (быстрый фидбек)
        print!("{}", formatted);

        // Запись в файл (персистентный лог)
        self.file
            .write_all(formatted.as_bytes())
            .expect("Failed to write log");
    }

    /// Лог уровня Info.
    ///
    /// Используется для:
    /// - старта системы
    /// - нормальных игровых событий
    /// - жизненного цикла приложения
    pub fn info(&mut self, msg: &str) {
        self.log(LogLevel::Info, msg);
    }

    /// Лог уровня Warn.
    ///
    /// Используется для:
    /// - подозрительных ситуаций
    /// - потенциальных проблем
    /// - fallback поведения
    pub fn warn(&mut self, msg: &str) {
        self.log(LogLevel::Warn, msg);
    }

    /// Лог уровня Error.
    ///
    /// Используется для:
    /// - ошибок загрузки ресурсов
    /// - сбоев логики
    /// - критических ситуаций
    pub fn error(&mut self, msg: &str) {
        self.log(LogLevel::Error, msg);
    }

    /// Лог уровня Debug.
    ///
    /// Используется для:
    /// - отладочной информации
    /// - спама при разработке
    /// - трассировки систем (ECS, input, AI)
    ///
    /// В релизе обычно отключается.
    pub fn debug(&mut self, msg: &str) {
        self.log(LogLevel::Debug, msg);
    }
    
}


/// Возвращает текущее время в формате HH:MM:SS.
///
/// Используется для:
/// - человекочитаемых логов
/// - быстрой навигации по времени событий
///
/// Пример:
/// "14:23:07"
fn current_time_string() -> String {
    let now = Local::now();
    now.format("%H:%M:%S").to_string()
}
 
/// Генерирует имя файла логов.
///
/// Формат:
/// logs/YYYYMMDD_HHMM_engine.log
///
/// Пример:
/// logs/20260505_1430_engine.log
///
/// Это позволяет:
/// - иметь отдельный лог на запуск
/// - удобно дебажить конкретные сессии
fn generate_log_filename() -> String {
    let now = Local::now();

    let formatted = now.format("%Y%m%d_%H%M").to_string();

    format!("logs/{}_engine.log", formatted)
}