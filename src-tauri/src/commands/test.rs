use crate::core::{
    api_result::{ApiError, ApiResult, ErrorCode},
    errors::{AppErrors, NetworkError},
};
use tauri::{command, App, AppHandle, Manager, State};

#[command]
pub fn test_command() -> ApiResult<String> {
    // Пример использования ApiResult
    let message = "Test command executed successfully";

    let is_error = true; // Замените на вашу логику проверки ошибок

    if is_error {
        // Если произошла ошибка, возвращаем ApiResult с ошибкой
        return AppErrors::Network(NetworkError::Timeout).into_api_result();
    }

    // Возвращаем успешный результат
    ApiResult::success(message.to_string())
}
