use crate::core::errors::{AppErrors, NetworkError, Result, ValidationError};
use tauri_plugin_http::reqwest::Client;

#[derive(Clone)]
pub struct HttpFetcher {
    client: Client,
    delay_ms: u64,
}

impl HttpFetcher {
    pub fn new(delay_ms: u64) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        HttpFetcher { client, delay_ms }
    }
    pub async fn fetch(&self, url: &str) -> Result<String> {
        // Валидация URL
        if url.is_empty() {
            return Err(AppErrors::Validation(ValidationError::ValidationFailed(
                "URL не может быть пустым".to_string(),
            )));
        }

        // Проверка формата URL
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppErrors::Network(NetworkError::InvalidUrl(
                url.to_string(),
            )));
        }

        // Добавляем задержку
        tokio::time::sleep(std::time::Duration::from_millis(self.delay_ms)).await;

        // Выполняем запрос
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| AppErrors::Network(NetworkError::from(e)))?;

        // Проверяем статус ответа
        if !response.status().is_success() {
            return Err(AppErrors::Network(NetworkError::RequestFailed(format!(
                "HTTP {}: {}",
                response.status(),
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("Unknown error")
            ))));
        }

        // Получаем текст ответа
        response.text().await.map_err(|e| {
            AppErrors::Network(NetworkError::RequestFailed(format!(
                "Ошибка чтения ответа: {}",
                e
            )))
        })
    }

    /// Скачивает файл и сохраняет его на диск
    pub async fn download_file(&self, url: &str, path: &str) -> Result<()> {
        let content = self.fetch(url).await?;

        std::fs::write(path, content).map_err(|e| {
            AppErrors::FileSystem(crate::core::errors::FileSystemError::IoError(e.to_string()))
        })?;

        Ok(())
    }

    /// Получает и парсит JSON
    pub async fn fetch_json<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let text = self.fetch(url).await?;

        serde_json::from_str(&text).map_err(|e| {
            AppErrors::Parsing(crate::core::errors::ParsingError::JsonError(e.to_string()))
        })
    }

    /// Проверяет доступность URL
    pub async fn check_url(&self, url: &str) -> Result<bool> {
        match self.client.head(url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                if e.is_timeout() {
                    Err(AppErrors::Network(NetworkError::Timeout))
                } else if e.is_connect() {
                    Err(AppErrors::Network(NetworkError::NoConnection))
                } else {
                    Err(AppErrors::Network(NetworkError::RequestFailed(
                        e.to_string(),
                    )))
                }
            }
        }
    }

    /// Пакетная загрузка нескольких URL
    pub async fn fetch_multiple(&self, urls: &[&str]) -> Result<Vec<String>> {
        if urls.is_empty() {
            return Err(AppErrors::Validation(ValidationError::ValidationFailed(
                "Список URL не может быть пустым".to_string(),
            )));
        }

        let mut results = Vec::new();

        for url in urls {
            match self.fetch(url).await {
                Ok(content) => results.push(content),
                Err(e) => {
                    // Логируем ошибку и продолжаем
                    eprintln!("Ошибка при загрузке {}: {}", url, e);
                    return Err(e);
                }
            }
        }

        Ok(results)
    }
}
