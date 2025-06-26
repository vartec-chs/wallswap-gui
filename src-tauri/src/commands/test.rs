use crate::core::{
    api_result::{
        ApiError, ApiResult, ApiSuccess, AppErrors, ErrorCode, NetworkError, SuccessCode,
        TauriResult,
    },
    models::{Category, ResolutionFilter},
};
use crate::states::{FilterState, MainState};
use tauri::{command, App, AppHandle, Manager, State};

#[command]
pub async fn test_command() -> TauriResult<()> {
    // Пример использования ApiResult
    let message = "Test command executed successfully";

    let is_error = false; // Замените на вашу логику проверки ошибок

    if is_error {
        // Если произошла ошибка, возвращаем ApiResult с ошибкой
        return Ok(ApiResult::Error(ApiError {
            code: ErrorCode::NetworkError,
            message: "An error occurred while executing the test command".to_string(),
            details: None,
        }));
    }

    // Возвращаем успешный результат
    Ok(ApiSuccess::no_code_void().into_api_result())
}

// #[command]
// pub fn get_main_state(app: AppHandle) -> ApiResult<MainState> {
// 	// Получаем состояние приложения
// 	let state = app.state::<MainState>().clone();

// 	// Возвращаем состояние в ApiResult
// 	ApiResult::success(state)
// }

// set category and resolution in filter state
#[command]
pub async fn set_filter_state(app: AppHandle, state: State<'_, FilterState>) -> TauriResult<()> {
    let categories = vec![
        Category {
            name: "Nature".to_string(),
            url: "https://example.com/nature".to_string(),
            count_wallpapers: Some(100),
        },
        Category {
            name: "Technology".to_string(),
            url: "https://example.com/technology".to_string(),
            count_wallpapers: Some(200),
        },
    ];

    // Устанавливаем выбранную категорию и разрешение

    // Возвращаем успешный результат
    Ok(ApiSuccess::no_code_void().into_api_result())
}
