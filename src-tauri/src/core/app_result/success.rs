use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SuccessCode {
    // CRUD операции
    Created = 201,
    Updated = 202,
    Deleted = 203,
    Retrieved = 200,

    // Операции с файлами
    FileUploaded = 210,
    FileDownloaded = 211,
    FileProcessed = 212,

    // Аутентификация
    LoginSuccessful = 220,
    LogoutSuccessful = 221,
    TokenRefreshed = 222,
    PasswordChanged = 223,

    // Системные операции
    ServiceStarted = 230,
    ServiceStopped = 231,
    BackupCompleted = 232,
    SyncCompleted = 233,

    // Бизнес операции
    PaymentProcessed = 240,
    OrderCompleted = 241,
    NotificationSent = 242,
    ReportGenerated = 243,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "success")]
pub struct AppSuccess<T> {
    pub code: SuccessCode,
    pub message: String,
    pub data: Option<T>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub execution_time_ms: Option<u64>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl<T> AppSuccess<T> {
    pub fn new(
        code: Option<SuccessCode>,
        message: Option<String>,
        data: Option<T>,
        execution_time_ms: Option<u64>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            code: code.unwrap_or(SuccessCode::Retrieved),
            message: message.unwrap_or("Success".to_string()),
            data,
            timestamp: chrono::Utc::now(),
            execution_time_ms,
            metadata,
        }
    }

    pub fn simple(message: String) -> Self {
        Self {
            code: SuccessCode::Retrieved,
            message,
            data: None,
            timestamp: chrono::Utc::now(),
            execution_time_ms: None,
            metadata: None,
        }
    }
}
