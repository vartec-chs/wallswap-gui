use thiserror::Error;

use super::*;

/// Основной enum для всех ошибок приложения
#[derive(Error, Debug, Clone)]
pub enum AppErrors {
    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    FileSystem(#[from] FileSystemError),

    #[error(transparent)]
    Parsing(#[from] ParsingError),

    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error(transparent)]
    NotFound(#[from] NotFoundError),

    #[error(transparent)]
    Operation(#[from] OperationError),

    #[error(transparent)]
    System(#[from] SystemError),

    #[error(transparent)]
    General(#[from] GeneralError),
}

impl AppErrors {
    /// Возвращает категорию ошибки
    pub fn category(&self) -> &'static str {
        match self {
            AppErrors::Network(_) => "network",
            AppErrors::FileSystem(_) => "file_system",
            AppErrors::Parsing(_) => "parsing",
            AppErrors::Validation(_) => "validation",
            AppErrors::NotFound(_) => "not_found",
            AppErrors::Operation(_) => "operation",
            AppErrors::System(_) => "system",
            AppErrors::General(_) => "general",
        }
    }

    /// Проверяет, можно ли повторить операцию при данной ошибке
    pub fn is_retryable(&self) -> bool {
        match self {
            AppErrors::Network(NetworkError::Timeout) => true,
            AppErrors::Network(NetworkError::RequestFailed(_)) => true,
            AppErrors::Operation(OperationError::DownloadError(_)) => true,
            AppErrors::General(GeneralError::Unknown) => true,
            _ => false,
        }
    }

    /// Возвращает уровень критичности ошибки
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppErrors::Network(_) => ErrorSeverity::Medium,
            AppErrors::FileSystem(_) => ErrorSeverity::High,
            AppErrors::Parsing(_) => ErrorSeverity::Medium,
            AppErrors::Validation(_) => ErrorSeverity::Low,
            AppErrors::NotFound(_) => ErrorSeverity::Low,
            AppErrors::Operation(_) => ErrorSeverity::Medium,
            AppErrors::System(_) => ErrorSeverity::High,
            AppErrors::General(GeneralError::Unknown) => ErrorSeverity::Critical,
            AppErrors::General(_) => ErrorSeverity::Medium,
        }
    }

    /// Преобразует в ApiResult
    pub fn into_api_result<T>(self) -> super::ApiResult<T> {
        super::ApiResult::Error(self.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}
