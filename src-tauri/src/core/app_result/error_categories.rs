// use serde::{Deserialize, Serialize};
use scraper::error::SelectorErrorKind;
use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;
use thiserror::Error;

/// Категория: Сетевые ошибки
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl NetworkError {
    pub fn error_type(&self) -> String {
        match self {
            NetworkError::Timeout => "Timeout".to_string(),
            NetworkError::NoConnection => "NoConnection".to_string(),
            NetworkError::RequestFailed(_) => "RequestFailed".to_string(),
            NetworkError::InvalidUrl(_) => "InvalidUrl".to_string(),
        }
    }
}

/// Категория: Файловые ошибки
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl FileSystemError {
    pub fn error_type(&self) -> String {
        match self {
            FileSystemError::IoError(_) => "IoError".to_string(),
            FileSystemError::WallpaperFileNotFound(_) => "WallpaperFileNotFound".to_string(),
            FileSystemError::FileSystemError(_) => "FileSystemError".to_string(),
            FileSystemError::AccessDenied(_) => "AccessDenied".to_string(),
            FileSystemError::FileCorrupted(_) => "FileCorrupted".to_string(),
        }
    }
}

/// Категория: Ошибки парсинга
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParsingError {
    #[error("Ошибка парсинга JSON: {0}")]
    JsonError(String),

    #[error("Ошибка парсинга HTML: {0}")]
    HtmlParseError(String),

    #[error("Ошибка сериализации: {0}")]
    SerializationError(String),

    #[error("Ошибка десериализации: {0}")]
    DeserializationError(String),

    #[error("Неверный формат данных: {0}")]
    InvalidFormat(String),
}

impl ParsingError {
    pub fn error_type(&self) -> String {
        match self {
            ParsingError::JsonError(_) => "JsonError".to_string(),
            ParsingError::HtmlParseError(_) => "HtmlParseError".to_string(),
            ParsingError::SerializationError(_) => "SerializationError".to_string(),
            ParsingError::DeserializationError(_) => "DeserializationError".to_string(),
            ParsingError::InvalidFormat(_) => "InvalidFormat".to_string(),
        }
    }
}

/// Категория: Ошибки валидации
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl ValidationError {
    pub fn error_type(&self) -> String {
        match self {
            ValidationError::ValidationFailed(_) => "ValidationFailed".to_string(),
            ValidationError::ConfigError(_) => "ConfigError".to_string(),
            ValidationError::MissingField(_) => "MissingField".to_string(),
            ValidationError::InvalidFieldValue { .. } => "InvalidFieldValue".to_string(),
        }
    }
}

/// Категория: Не найдено
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl NotFoundError {
    pub fn error_type(&self) -> String {
        match self {
            NotFoundError::CategoriesNotFound => "CategoriesNotFound".to_string(),
            NotFoundError::WallpaperNotFound => "WallpaperNotFound".to_string(),
            NotFoundError::HistoryEmpty => "HistoryEmpty".to_string(),
            NotFoundError::ResourceNotFound(_) => "ResourceNotFound".to_string(),
        }
    }
}

/// Категория: Операционные ошибки
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl OperationError {
    pub fn error_type(&self) -> String {
        match self {
            OperationError::DownloadError(_) => "DownloadError".to_string(),
            OperationError::WallpaperSetError(_) => "WallpaperSetError".to_string(),
            OperationError::Cancelled => "Cancelled".to_string(),
            OperationError::UnsupportedOperation(_) => "UnsupportedOperation".to_string(),
            OperationError::OperationInProgress => "OperationInProgress".to_string(),
        }
    }
}

/// Категория: Системные ошибки
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

impl SystemError {
    pub fn error_type(&self) -> String {
        match self {
            SystemError::UnsupportedOS => "UnsupportedOS".to_string(),
            SystemError::InsufficientPermissions => "InsufficientPermissions".to_string(),
            SystemError::InsufficientSpace => "InsufficientSpace".to_string(),
            SystemError::ResourceUnavailable(_) => "ResourceUnavailable".to_string(),
        }
    }
}

/// Категория: Общие ошибки
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GeneralError {
    #[error("Ошибка: {0}")]
    Other(String),

    #[error("Неизвестная ошибка")]
    Unknown,

    #[error("Внутренняя ошибка: {0}")]
    InternalError(String),
}

impl GeneralError {
    pub fn error_type(&self) -> String {
        match self {
            GeneralError::Other(_) => "Other".to_string(),
            GeneralError::Unknown => "Unknown".to_string(),
            GeneralError::InternalError(_) => "InternalError".to_string(),
        }
    }
}

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

impl<'a> From<SelectorErrorKind<'a>> for ParsingError {
    fn from(e: SelectorErrorKind<'a>) -> Self {
        ParsingError::HtmlParseError(e.to_string())
    }
}
