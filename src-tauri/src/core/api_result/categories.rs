use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;
use thiserror::Error;

/// Категория: Сетевые ошибки
#[derive(Error, Debug, Clone)]
pub enum NetworkError {
    #[error("Ошибка сети: {0}")]
    RequestFailed(String),

    #[error("Таймаут соединения")]
    Timeout,

    #[error("Нет подключения к интернету")]
    NoConnection,

    #[error("Неверный URL: {0}")]
    InvalidUrl(String),
}

/// Категория: Файловые ошибки
#[derive(Error, Debug, Clone)]
pub enum FileSystemError {
    #[error("Ошибка ввода/вывода: {0}")]
    IoError(String),

    #[error("Файл обоев не найден: {0}")]
    WallpaperFileNotFound(String),

    #[error("Ошибка файловой системы: {0}")]
    FileSystemError(String),

    #[error("Доступ к файлу запрещен: {0}")]
    AccessDenied(String),

    #[error("Файл поврежден: {0}")]
    FileCorrupted(String),
}

/// Категория: Ошибки парсинга
#[derive(Error, Debug, Clone)]
pub enum ParsingError {
    #[error("Ошибка парсинга JSON: {0}")]
    JsonError(String),

    #[error("Ошибка парсинга HTML")]
    HtmlParseError,

    #[error("Ошибка сериализации: {0}")]
    SerializationError(String),

    #[error("Ошибка десериализации: {0}")]
    DeserializationError(String),

    #[error("Неверный формат данных: {0}")]
    InvalidFormat(String),
}

/// Категория: Ошибки валидации
#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Ошибка валидации: {0}")]
    ValidationFailed(String),

    #[error("Ошибка конфигурации: {0}")]
    ConfigError(String),

    #[error("Отсутствует обязательное поле: {0}")]
    MissingField(String),

    #[error("Неверное значение поля {field}: {value}")]
    InvalidFieldValue { field: String, value: String },
}

/// Категория: Не найдено
#[derive(Error, Debug, Clone)]
pub enum NotFoundError {
    #[error("Категории не найдены")]
    CategoriesNotFound,

    #[error("Обои не найдены")]
    WallpaperNotFound,

    #[error("История пуста")]
    HistoryEmpty,

    #[error("Ресурс не найден: {0}")]
    ResourceNotFound(String),
}

/// Категория: Операционные ошибки
#[derive(Error, Debug, Clone)]
pub enum OperationError {
    #[error("Ошибка загрузки: {0}")]
    DownloadError(String),

    #[error("Ошибка установки обоев: {0}")]
    WallpaperSetError(String),

    #[error("Операция отменена")]
    Cancelled,

    #[error("Операция не поддерживается: {0}")]
    UnsupportedOperation(String),

    #[error("Операция уже выполняется")]
    OperationInProgress,
}

/// Категория: Системные ошибки
#[derive(Error, Debug, Clone)]
pub enum SystemError {
    #[error("Неподдерживаемая операционная система")]
    UnsupportedOS,

    #[error("Недостаточно прав доступа")]
    InsufficientPermissions,

    #[error("Недостаточно места на диске")]
    InsufficientSpace,

    #[error("Системный ресурс недоступен: {0}")]
    ResourceUnavailable(String),
}

/// Категория: Общие ошибки
#[derive(Error, Debug, Clone)]
pub enum GeneralError {
    #[error("Ошибка: {0}")]
    Other(String),

    #[error("Неизвестная ошибка")]
    Unknown,

    #[error("Внутренняя ошибка: {0}")]
    InternalError(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorDetails {
    pub category: String,
    pub error_type: String,
    pub retryable: bool,
    pub nested_details: Option<serde_json::Value>,
}
 
// Добавляем поддержку для асинхронных команд Tauri
// impl<T> tauri::async_runtime::A for crate::core::api_result::ApiResult<T>
// where
//     T: serde::Serialize + Send + 'static,
// {
//     type Output = Self;

//     fn from_output(output: Self::Output) -> Self {
//         output
//     }
// }

// Конверторы из стандартных ошибок
impl From<reqwest::Error> for NetworkError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_timeout() {
            NetworkError::Timeout
        } else if error.is_connect() {
            NetworkError::NoConnection
        } else {
            NetworkError::RequestFailed(error.to_string())
        }
    }
}

impl From<std::io::Error> for FileSystemError {
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            std::io::ErrorKind::NotFound => {
                FileSystemError::WallpaperFileNotFound(error.to_string())
            }
            std::io::ErrorKind::PermissionDenied => {
                FileSystemError::AccessDenied(error.to_string())
            }
            _ => FileSystemError::IoError(error.to_string()),
        }
    }
}

impl From<serde_json::Error> for ParsingError {
    fn from(error: serde_json::Error) -> Self {
        ParsingError::JsonError(error.to_string())
    }
}
