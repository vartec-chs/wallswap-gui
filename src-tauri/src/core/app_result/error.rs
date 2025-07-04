use super::error_categories::{
    FileSystemError, GeneralError, NetworkError, NotFoundError, OperationError, ParsingError,
    SystemError, ValidationError,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AppError {
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    #[error("File system error: {0}")]
    FileSystem(#[from] FileSystemError),

    #[error("Parsing error: {0}")]
    Parsing(#[from] ParsingError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("General error: {0}")]
    General(#[from] GeneralError),

    #[error("Not found error: {0}")]
    NotFound(#[from] NotFoundError),

    #[error("Operation error: {0}")]
    Operation(#[from] OperationError),

    #[error("System error: {0}")]
    System(#[from] SystemError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "error")]
pub struct ErrorDetails {
    pub category: String,
    pub error_type: String,
    pub severity: ErrorSeverity,
    pub retryable: bool,
    pub message: String,
    pub full_message: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub trace_id: Option<String>,
    pub nested_details: Option<HashMap<String, serde_json::Value>>,
}

impl ErrorDetails {
    pub fn new(
        message: String,
        nested_details: Option<HashMap<String, serde_json::Value>>,
        trace_id: Option<String>,
    ) -> Self {
        Self {
            category: "GeneralError".to_string(),
            error_type: "Unknown".to_string(),
            severity: ErrorSeverity::Medium,
            retryable: false,
            message,
            full_message: "An error occurred".to_string(),
            timestamp: Utc::now(),
            trace_id,
            nested_details,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl AppError {
    pub fn to_error_details(&self, trace_id: Option<String>) -> ErrorDetails {
        let trace_id = trace_id.or_else(|| {
            std::thread::current()
                .name()
                .map(|name| name.to_string())
                .or_else(|| Some("unknown".to_string()))
        });

        ErrorDetails {
            category: self.category(),
            error_type: self.error_type(),
            retryable: self.retryable(),
            severity: self.severity(),
            message: self.message().to_string(),
            full_message: self.full_message(),
            timestamp: Utc::now(),
            trace_id,
            nested_details: None,
        }
    }

    pub fn category(&self) -> String {
        match self {
            AppError::Network(_) => "NetworkError".to_string(),
            AppError::FileSystem(_) => "FileSystemError".to_string(),
            AppError::Parsing(_) => "ParsingError".to_string(),
            AppError::Validation(_) => "ValidationError".to_string(),
            AppError::General(_) => "GeneralError".to_string(),
            AppError::NotFound(_) => "NotFoundError".to_string(),
            AppError::Operation(_) => "OperationError".to_string(),
            AppError::System(_) => "SystemError".to_string(),
        }
    }

    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::Network(_) => ErrorSeverity::Medium,
            AppError::FileSystem(_) => ErrorSeverity::High,
            AppError::Parsing(_) => ErrorSeverity::Medium,
            AppError::Validation(_) => ErrorSeverity::Low,
            AppError::General(_) => ErrorSeverity::Medium,
            AppError::NotFound(_) => ErrorSeverity::Low,
            AppError::Operation(_) => ErrorSeverity::High,
            AppError::System(_) => ErrorSeverity::Critical,
        }
    }

    pub fn retryable(&self) -> bool {
        match self {
            AppError::Network(_) => true,
            AppError::FileSystem(_) => false,
            AppError::Parsing(_) => false,
            AppError::Validation(_) => false,
            AppError::General(_) => false,
            AppError::NotFound(_) => false,
            AppError::Operation(_) => true,
            AppError::System(_) => false,
        }
    }

    pub fn error_type(&self) -> String {
        match self {
            AppError::Network(e) => e.error_type(),
            AppError::FileSystem(e) => e.error_type(),
            AppError::Parsing(e) => e.error_type(),
            AppError::Validation(e) => e.error_type(),
            AppError::General(e) => e.error_type(),
            AppError::NotFound(e) => e.error_type(),
            AppError::Operation(e) => e.error_type(),
            AppError::System(e) => e.error_type(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            AppError::Network(e) => e.to_string(),
            AppError::FileSystem(e) => e.to_string(),
            AppError::Parsing(e) => e.to_string(),
            AppError::Validation(e) => e.to_string(),
            AppError::General(e) => e.to_string(),
            AppError::NotFound(e) => e.to_string(),
            AppError::Operation(e) => e.to_string(),
            AppError::System(e) => e.to_string(),
        }
    }

    pub fn full_message(&self) -> String {
        self.to_string()
    }
}
