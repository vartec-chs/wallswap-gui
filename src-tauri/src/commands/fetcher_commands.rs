use crate::core::{
    errors::{AppErrors, Result},
    fetcher::HttpFetcher,
    utils::api_result::{ApiResult, SuccessCode},
};
use tauri::State;

// Пример Tauri команды с использованием системы ошибок
#[tauri::command]
pub async fn fetch_wallpaper_categories(
    fetcher: State<'_, HttpFetcher>,
) -> std::result::Result<ApiResult<Vec<String>>, String> {
    let url = "https://example.com/api/categories";

    match fetcher.fetch_json::<Vec<String>>(url).await {
        Ok(categories) => {
            if categories.is_empty() {
                Ok(
                    AppErrors::NotFound(crate::core::errors::NotFoundError::CategoriesNotFound)
                        .into(),
                )
            } else {
                Ok(ApiResult::success_with_code(
                    Some(SuccessCode::DataRetrieved),
                    categories,
                ))
            }
        }
        Err(e) => {
            // Логируем ошибку
            eprintln!("Ошибка при получении категорий: {}", e);

            // Конвертируем AppErrors в ApiResult
            Ok(e.into())
        }
    }
}

#[tauri::command]
pub async fn download_wallpaper(
    fetcher: State<'_, HttpFetcher>,
    url: String,
    save_path: String,
) -> std::result::Result<ApiResult<String>, String> {
    match fetcher.download_file(&url, &save_path).await {
        Ok(_) => Ok(ApiResult::success_with_code(
            Some(SuccessCode::OperationSuccessful),
            format!("Обои успешно скачаны в: {}", save_path),
        )),
        Err(e) => {
            eprintln!("Ошибка при скачивании обоев: {}", e);
            Ok(e.into())
        }
    }
}

#[tauri::command]
pub async fn check_connection(
    fetcher: State<'_, HttpFetcher>,
    test_url: String,
) -> std::result::Result<ApiResult<bool>, String> {
    match fetcher.check_url(&test_url).await {
        Ok(is_available) => Ok(ApiResult::success(is_available)),
        Err(e) => {
            eprintln!("Ошибка при проверке соединения: {}", e);
            Ok(e.into())
        }
    }
}

// Пример обработки ошибок с retry логикой
#[tauri::command]
pub async fn fetch_with_retry(
    fetcher: State<'_, HttpFetcher>,
    url: String,
    max_retries: u32,
) -> std::result::Result<ApiResult<String>, String> {
    let mut attempts = 0;

    loop {
        attempts += 1;

        match fetcher.fetch(&url).await {
            Ok(content) => {
                return Ok(ApiResult::success_with_code(
                    Some(SuccessCode::DataRetrieved),
                    content,
                ));
            }
            Err(e) => {
                // Проверяем, можно ли повторить операцию
                if e.is_retryable() && attempts <= max_retries {
                    eprintln!(
                        "Попытка {} не удалась, повторяем... Ошибка: {}",
                        attempts, e
                    );
                    tokio::time::sleep(std::time::Duration::from_millis(1000 * attempts as u64))
                        .await;
                    continue;
                } else {
                    eprintln!(
                        "Не удалось загрузить после {} попыток. Ошибка: {}",
                        attempts, e
                    );
                    return Ok(e.into());
                }
            }
        }
    }
}
