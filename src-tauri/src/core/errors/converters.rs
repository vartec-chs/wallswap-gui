use crate::core::{
    errors::{
        AppErrors, ErrorDetails, FileSystemError, GeneralError, NetworkError, NotFoundError,
        OperationError, ParsingError, SystemError, ValidationError,
    },
    utils::api_result::{ApiError, ApiResult, ErrorCode},
};

/// Конвертер из AppErrors в ApiError
impl From<AppErrors> for ApiError {
    fn from(error: AppErrors) -> Self {
        let (code, message, details) = match error {
            AppErrors::Network(e) => convert_network_error(e),
            AppErrors::FileSystem(e) => convert_filesystem_error(e),
            AppErrors::Parsing(e) => convert_parsing_error(e),
            AppErrors::Validation(e) => convert_validation_error(e),
            AppErrors::NotFound(e) => convert_notfound_error(e),
            AppErrors::Operation(e) => convert_operation_error(e),
            AppErrors::System(e) => convert_system_error(e),
            AppErrors::General(e) => convert_general_error(e),
        };

        ApiError {
            code,
            message,
            details,
        }
    }
}

/// Конвертер из AppErrors в ApiResult
impl<T> From<AppErrors> for ApiResult<T> {
    fn from(error: AppErrors) -> Self {
        ApiResult::Error(error.into())
    }
}

// Функции для конвертации каждой категории ошибок

fn convert_network_error(error: NetworkError) -> (ErrorCode, String, Option<ErrorDetails>) {
    match error {
        NetworkError::RequestFailed(msg) => (
            ErrorCode::NetworkError,
            format!("Ошибка сети: {}", msg),
            Some(ErrorDetails {
                category: "network".to_string(),
                error_type: "request_failed".to_string(),
                retryable: true,
                nested_details: Some(serde_json::json!({
                    "original_error": msg
                })),
            }),
        ),
        NetworkError::Timeout => (
            ErrorCode::Timeout,
            "Таймаут соединения".to_string(),
            Some(ErrorDetails {
                category: "network".to_string(),
                error_type: "timeout".to_string(),
                retryable: true,
                nested_details: None,
            }),
        ),
        NetworkError::NoConnection => (
            ErrorCode::NetworkError,
            "Нет подключения к интернету".to_string(),
            Some(ErrorDetails {
                category: "network".to_string(),
                error_type: "no_connection".to_string(),
                retryable: true,
                nested_details: None,
            }),
        ),
        NetworkError::InvalidUrl(url) => (
            ErrorCode::InvalidInput,
            format!("Неверный URL: {}", url),
            Some(ErrorDetails {
                category: "network".to_string(),
                error_type: "invalid_url".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "url": url
                })),
            }),
        ),
    }
}

fn convert_filesystem_error(error: FileSystemError) -> (ErrorCode, String, Option<ErrorDetails>) {
    match error {
        FileSystemError::IoError(msg) => (
            ErrorCode::FileAccessDenied,
            format!("Ошибка ввода/вывода: {}", msg),
            Some(ErrorDetails {
                category: "file_system".to_string(),
                error_type: "io_error".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        FileSystemError::WallpaperFileNotFound(path) => (
            ErrorCode::FileNotFound,
            format!("Файл обоев не найден: {}", path),
            Some(ErrorDetails {
                category: "file_system".to_string(),
                error_type: "wallpaper_file_not_found".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "file_path": path
                })),
            }),
        ),
        FileSystemError::FileSystemError(msg) => (
            ErrorCode::FileAccessDenied,
            format!("Ошибка файловой системы: {}", msg),
            Some(ErrorDetails {
                category: "file_system".to_string(),
                error_type: "filesystem_error".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        FileSystemError::AccessDenied(path) => (
            ErrorCode::FileAccessDenied,
            format!("Доступ к файлу запрещен: {}", path),
            Some(ErrorDetails {
                category: "file_system".to_string(),
                error_type: "access_denied".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "file_path": path
                })),
            }),
        ),
        FileSystemError::FileCorrupted(path) => (
            ErrorCode::FileCorrupted,
            format!("Файл поврежден: {}", path),
            Some(ErrorDetails {
                category: "file_system".to_string(),
                error_type: "file_corrupted".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "file_path": path
                })),
            }),
        ),
    }
}

fn convert_parsing_error(error: ParsingError) -> (ErrorCode, String, Option<ErrorDetails>) {
    match error {
        ParsingError::JsonError(msg) => (
            ErrorCode::InvalidInput,
            format!("Ошибка парсинга JSON: {}", msg),
            Some(ErrorDetails {
                category: "parsing".to_string(),
                error_type: "json_parse".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        ParsingError::HtmlParseError => (
            ErrorCode::InvalidInput,
            "Ошибка парсинга HTML".to_string(),
            Some(ErrorDetails {
                category: "parsing".to_string(),
                error_type: "html_parse".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        ParsingError::SerializationError(msg) => (
            ErrorCode::InvalidInput,
            format!("Ошибка сериализации: {}", msg),
            Some(ErrorDetails {
                category: "parsing".to_string(),
                error_type: "serialization".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        ParsingError::DeserializationError(msg) => (
            ErrorCode::InvalidInput,
            format!("Ошибка десериализации: {}", msg),
            Some(ErrorDetails {
                category: "parsing".to_string(),
                error_type: "deserialization".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        ParsingError::InvalidFormat(msg) => (
            ErrorCode::InvalidInput,
            format!("Неверный формат данных: {}", msg),
            Some(ErrorDetails {
                category: "parsing".to_string(),
                error_type: "invalid_format".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
    }
}

fn convert_validation_error(error: ValidationError) -> (ErrorCode, String, Option<ErrorDetails>) {
    match error {
        ValidationError::ValidationFailed(msg) => (
            ErrorCode::ValidationFailed,
            format!("Ошибка валидации: {}", msg),
            Some(ErrorDetails {
                category: "validation".to_string(),
                error_type: "validation_failed".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        ValidationError::ConfigError(msg) => (
            ErrorCode::InvalidInput,
            format!("Ошибка конфигурации: {}", msg),
            Some(ErrorDetails {
                category: "validation".to_string(),
                error_type: "configuration".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        ValidationError::MissingField(field) => (
            ErrorCode::ValidationFailed,
            format!("Отсутствует обязательное поле: {}", field),
            Some(ErrorDetails {
                category: "validation".to_string(),
                error_type: "missing_field".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "field": field
                })),
            }),
        ),
        ValidationError::InvalidFieldValue { field, value } => (
            ErrorCode::ValidationFailed,
            format!("Неверное значение поля {}: {}", field, value),
            Some(ErrorDetails {
                category: "validation".to_string(),
                error_type: "invalid_field_value".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "field": field,
                    "value": value
                })),
            }),
        ),
    }
}

fn convert_notfound_error(error: NotFoundError) -> (ErrorCode, String, Option<ErrorDetails>) {
    match error {
        NotFoundError::CategoriesNotFound => (
            ErrorCode::NotFound,
            "Категории не найдены".to_string(),
            Some(ErrorDetails {
                category: "not_found".to_string(),
                error_type: "categories_not_found".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        NotFoundError::WallpaperNotFound => (
            ErrorCode::NotFound,
            "Обои не найдены".to_string(),
            Some(ErrorDetails {
                category: "not_found".to_string(),
                error_type: "wallpaper_not_found".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        NotFoundError::HistoryEmpty => (
            ErrorCode::NotFound,
            "История пуста".to_string(),
            Some(ErrorDetails {
                category: "not_found".to_string(),
                error_type: "history_empty".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        NotFoundError::ResourceNotFound(resource) => (
            ErrorCode::NotFound,
            format!("Ресурс не найден: {}", resource),
            Some(ErrorDetails {
                category: "not_found".to_string(),
                error_type: "resource_not_found".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "resource": resource
                })),
            }),
        ),
    }
}

fn convert_operation_error(error: OperationError) -> (ErrorCode, String, Option<ErrorDetails>) {
    match error {
        OperationError::DownloadError(msg) => (
            ErrorCode::OperationFailed,
            format!("Ошибка загрузки: {}", msg),
            Some(ErrorDetails {
                category: "operation".to_string(),
                error_type: "download_failed".to_string(),
                retryable: true,
                nested_details: None,
            }),
        ),
        OperationError::WallpaperSetError(msg) => (
            ErrorCode::OperationFailed,
            format!("Ошибка установки обоев: {}", msg),
            Some(ErrorDetails {
                category: "operation".to_string(),
                error_type: "wallpaper_set_failed".to_string(),
                retryable: true,
                nested_details: None,
            }),
        ),
        OperationError::Cancelled => (
            ErrorCode::OperationFailed,
            "Операция отменена".to_string(),
            Some(ErrorDetails {
                category: "operation".to_string(),
                error_type: "cancelled".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        OperationError::UnsupportedOperation(op) => (
            ErrorCode::OperationFailed,
            format!("Операция не поддерживается: {}", op),
            Some(ErrorDetails {
                category: "operation".to_string(),
                error_type: "unsupported_operation".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "operation": op
                })),
            }),
        ),
        OperationError::OperationInProgress => (
            ErrorCode::OperationFailed,
            "Операция уже выполняется".to_string(),
            Some(ErrorDetails {
                category: "operation".to_string(),
                error_type: "operation_in_progress".to_string(),
                retryable: true,
                nested_details: None,
            }),
        ),
    }
}

fn convert_system_error(error: SystemError) -> (ErrorCode, String, Option<ErrorDetails>) {
    match error {
        SystemError::UnsupportedOS => (
            ErrorCode::OperationFailed,
            "Неподдерживаемая операционная система".to_string(),
            Some(ErrorDetails {
                category: "system".to_string(),
                error_type: "unsupported_os".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        SystemError::InsufficientPermissions => (
            ErrorCode::Forbidden,
            "Недостаточно прав доступа".to_string(),
            Some(ErrorDetails {
                category: "system".to_string(),
                error_type: "insufficient_permissions".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        SystemError::InsufficientSpace => (
            ErrorCode::OperationFailed,
            "Недостаточно места на диске".to_string(),
            Some(ErrorDetails {
                category: "system".to_string(),
                error_type: "insufficient_space".to_string(),
                retryable: false,
                nested_details: None,
            }),
        ),
        SystemError::ResourceUnavailable(resource) => (
            ErrorCode::OperationFailed,
            format!("Системный ресурс недоступен: {}", resource),
            Some(ErrorDetails {
                category: "system".to_string(),
                error_type: "resource_unavailable".to_string(),
                retryable: true,
                nested_details: Some(serde_json::json!({
                    "resource": resource
                })),
            }),
        ),
    }
}

fn convert_general_error(error: GeneralError) -> (ErrorCode, String, Option<ErrorDetails>) {
    match error {
        GeneralError::Other(msg) => (
            ErrorCode::InternalError,
            msg.clone(),
            Some(ErrorDetails {
                category: "general".to_string(),
                error_type: "other".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "message": msg
                })),
            }),
        ),
        GeneralError::Unknown => (
            ErrorCode::InternalError,
            "Неизвестная ошибка".to_string(),
            Some(ErrorDetails {
                category: "general".to_string(),
                error_type: "unknown".to_string(),
                retryable: true,
                nested_details: None,
            }),
        ),
        GeneralError::InternalError(msg) => (
            ErrorCode::InternalError,
            format!("Внутренняя ошибка: {}", msg),
            Some(ErrorDetails {
                category: "general".to_string(),
                error_type: "internal_error".to_string(),
                retryable: false,
                nested_details: Some(serde_json::json!({
                    "message": msg
                })),
            }),
        ),
    }
}
