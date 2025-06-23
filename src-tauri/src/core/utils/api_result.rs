use serde::{Deserialize, Serialize};

use crate::core::ErrorDetails;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "answer")]
pub enum ApiResult<T> {
    #[serde(rename = "success")]
    Success(ApiSuccess<T>),
    #[serde(rename = "error")]
    Error(ApiError),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiSuccess<T> {
    pub code: SuccessCode,
    pub data: T,
}

pub type ApiResultVoid = ApiResult<()>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<ErrorDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    // Общие ошибки
    InternalError,
    InvalidInput,
    NotFound,
    Unauthorized,
    Forbidden,

    // Пользовательские ошибки
    UserNotFound,
    UserAlreadyExists,
    InvalidCredentials,

    // Файловые ошибки
    FileNotFound,
    FileAccessDenied,
    FileCorrupted,

    // Сетевые ошибки
    NetworkError,
    Timeout,

    // Бизнес-логика
    ValidationFailed,
    OperationFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SuccessCode {
    OperationSuccessful,
    DataRetrieved,
    UserCreated,
    UserUpdated,
}

impl<T> ApiResult<T> {
    pub fn success_with_code(code: Option<SuccessCode>, data: T) -> Self {
        let code = code.unwrap_or(SuccessCode::OperationSuccessful);
        ApiResult::Success(ApiSuccess { code, data })
    }

    pub fn success(data: T) -> Self {
        ApiResult::success_with_code(None, data)
    }

    pub fn error(code: ErrorCode, message: impl Into<String>) -> Self {
        ApiResult::Error(ApiError {
            code,
            message: message.into(),
            details: None,
        })
    }

    pub fn error_with_details(
        code: ErrorCode,
        message: impl Into<String>,
        details: ErrorDetails,
    ) -> Self {
        ApiResult::Error(ApiError {
            code,
            message: message.into(),
            details: Some(details),
        })
    }

    pub fn is_success(&self) -> bool {
        matches!(self, ApiResult::Success(_))
    }

    pub fn is_error(&self) -> bool {
        matches!(self, ApiResult::Error(_))
    }
}

// Конвертация из стандартного Result
impl<T, E> From<Result<T, E>> for ApiResult<T>
where
    E: std::fmt::Display,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(data) => ApiResult::success(data),
            Err(e) => ApiResult::error(ErrorCode::InternalError, e.to_string()),
        }
    }
}
