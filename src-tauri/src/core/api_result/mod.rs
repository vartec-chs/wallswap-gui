mod app_errors;
mod categories;
mod converters;

pub use app_errors::*;
pub use categories::*;
pub use converters::*;

pub type Result<T> = std::result::Result<T, AppErrors>;

use serde::{Deserialize, Serialize};

use crate::core::api_result::ErrorDetails;

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
// #[serde(tag = "success")]
pub struct ApiSuccess<T = ()> {
    pub code: SuccessCode,
    pub data: T,
}

impl<T> ApiSuccess<T> {
    pub fn new(code: SuccessCode, data: T) -> Self {
        ApiSuccess { code, data }
    }

    pub fn with_code(code: SuccessCode) -> impl FnOnce(T) -> Self {
        move |data| ApiSuccess { code, data }
    }

    pub fn no_code(data: T) -> Self {
        ApiSuccess {
            code: SuccessCode::OperationSuccessful,
            data,
        }
    }

    pub fn into_api_result(self) -> ApiResult<T> {
        ApiResult::Success(self)
    }
}

impl ApiSuccess<()> {
    pub fn new_void(code: SuccessCode) -> Self {
        ApiSuccess { code, data: () }
    }

    pub fn no_code_void() -> Self {
        ApiSuccess::new_void(SuccessCode::OperationSuccessful)
    }

    // pub fn into_api_result_void(self) -> ApiResultVoid {
    //     ApiResult::Ok(self)
    // }
}

// pub type ApiResult<T> = std::result::Result<ApiSuccess<T>, ApiError>;
// pub type ApiResultVoid = ApiResult<()>;
pub type TauriResult<T> = std::result::Result<ApiResult<T>, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
// #[serde(tag = "error")]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<ErrorDetails>,
}

impl ApiError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        ApiError {
            code,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(
        code: ErrorCode,
        message: impl Into<String>,
        details: ErrorDetails,
    ) -> Self {
        ApiError {
            code,
            message: message.into(),
            details: Some(details),
        }
    }
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

// impl<T> ApiResult<T> {
//     pub fn success_with_code(code: Option<SuccessCode>, data: T) -> Self {
//         let code = code.unwrap_or(SuccessCode::OperationSuccessful);
//         ApiResult::Ok(ApiSuccess { code, data })
//     }

//     pub fn success(data: T) -> Self {
//         ApiResult::success_with_code(None, data)
//     }

//     pub fn error(code: ErrorCode, message: impl Into<String>) -> Self {
//         ApiResult::Err(ApiError {
//             code,
//             message: message.into(),
//             details: None,
//         })
//     }

//     pub fn error_with_details(
//         code: ErrorCode,
//         message: impl Into<String>,
//         details: ErrorDetails,
//     ) -> Self {
//         ApiResult::Err(ApiError {
//             code,
//             message: message.into(),
//             details: Some(details),
//         })
//     }

//     pub fn is_success(&self) -> bool {
//         matches!(self, ApiResult::Ok(_))
//     }

//     pub fn is_error(&self) -> bool {
//         matches!(self, ApiResult::Err(_))
//     }
// }

// // Конвертация из стандартного Result
// impl<T, E> From<std::result::Result<T, E>> for ApiResult<T>
// where
//     E: std::fmt::Display,
// {
//     fn from(result: std::result::Result<T, E>) -> Self {
//         match result {
//             Ok(data) => ApiResult::success(data),
//             Err(e) => ApiResult::error(ErrorCode::InternalError, e.to_string()),
//         }
//     }
// }
