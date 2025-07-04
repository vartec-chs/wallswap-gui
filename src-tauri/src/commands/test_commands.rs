use tauri::{command, AppHandle, Manager, State};

use crate::core::app_result::{AppError, AppResult, NotFoundError, AppSuccess};

#[command]
pub async fn test_command(app: AppHandle) -> AppResult<String> {
    // Access the app handle and state

	let is_error = true; // Simulate a condition for error handling
	if is_error {
		return Err(AppError::NotFound(NotFoundError::CategoriesNotFound).to_error_details(None));
	}

    // Perform some operation
    let message = format!("Test command executed with state");
    Ok(AppSuccess::simple(message))
}
