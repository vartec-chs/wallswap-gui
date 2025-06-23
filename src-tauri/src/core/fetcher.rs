use std::time::Duration;

use crate::core::errors::{AppErrors, NetworkError, Result, ValidationError};
use log;
use tauri_plugin_http::reqwest::{header, Client, Request, Response};
use tokio::time::sleep;

#[derive(Clone)]
pub struct HttpFetcher {
    client: Client,
    delay_ms: u64,
}

impl HttpFetcher {
    pub fn new(user_agent: &str, timeout_secs: u64) -> Result<Self> {
        let client = Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(timeout_secs))
            .gzip(true) // Автоматическая декомпрессия gzip
            .deflate(true) // Автоматическая декомпрессия deflate
            .brotli(true) // Автоматическая декомпрессия brotli
            .build()
            .map_err(|e| {
                AppErrors::Network(NetworkError::RequestFailed(format!(
                    "ошибка создания клиента: {}",
                    e
                )))
            })?;

        Ok(Self {
            client,
            delay_ms: 1000, // Задержка по умолчанию 1 с
        })
    }

    pub fn with_delay(user_agent: &str, timeout_secs: u64, delay_ms: u64) -> Result<Self> {
        let client = Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(timeout_secs))
            .gzip(true)
            .deflate(true)
            .brotli(true)
            // Добавляем дополнительные заголовки для уменьшения вероятности блокировки
            .default_headers({
                let mut headers = header::HeaderMap::new();
                headers.insert(
                    "Accept",
                    "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"
                        .parse()
                        .unwrap(),
                );
                headers.insert(
                    "Accept-Language",
                    "ru-RU,ru;q=0.9,en;q=0.8".parse().unwrap(),
                );
                headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
                headers.insert("DNT", "1".parse().unwrap());
                headers.insert("Connection", "keep-alive".parse().unwrap());
                headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
                headers
            })
            .build()
            .map_err(|e| {
                AppErrors::Network(NetworkError::RequestFailed(format!(
                    "ошибка создания клиента: {}",
                    e
                )))
            })?;

        Ok(Self { client, delay_ms })
    }

    pub fn set_delay(&mut self, delay_ms: u64) {
        self.delay_ms = delay_ms;
    }

    pub async fn fetch(&self, url: &str) -> Result<Response> {
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
        if response.status().is_success() {
            Ok(response)
        } else {
            // self.logger
            //     .error(&format!("Ошибка HTTP: {}", response.status()));
            log::error!("Ошибка HTTP: {}", response.status());
            Err(AppErrors::Network(NetworkError::from(
                response.error_for_status().unwrap_err(),
            )))
        }

        // Получаем текст ответа
    }

    pub async fn fetch_with_retry(&self, url: &str, max_retries: u32) -> Result<Response> {
        let mut retries = 0;
        let mut delay = self.delay_ms;

        log::info!("Начинаем загрузку URL: {}", url);

        loop {
            match self.fetch(url).await {
                Ok(response) => {
                    log::info!("Успешно загружен URL: {}", url);
                    // Проверяем, что ответ не пустой
                    if response.content_length() == Some(0) {
                        log::warn!("Ответ от сервера пустой для URL: {}", url);
                        return Err(AppErrors::Network(NetworkError::RequestFailed(
                            "Ответ от сервера пустой".to_string(),
                        )));
                    }
                    // Возвращаем успешный ответ
                    return Ok(response);
                }
                Err(e) => {
                    log::error!("Ошибка при загрузке {}: {}", url, e);
                    if retries < max_retries {
                        if let AppErrors::Network(ref network_err) = e {
                            // Check if this is a 429 error by examining the error message or type
                            if network_err.to_string().contains("429") {
                                retries += 1;
                                // self.logger.warn(&format!(
                                //     "⏳ Получена ошибка 429, попытка {} из {}. Ждем {} мс...",
                                //     retries, max_retries, delay
                                // ));

                                log::warn!(
                                    "⏳ Получена ошибка 429, попытка {} из {}. Ждем {} мс...",
                                    retries,
                                    max_retries,
                                    delay
                                );

                                sleep(Duration::from_millis(delay)).await;

                                // Увеличиваем задержку для следующей попытки (экспоненциальная задержка)
                                delay *= 2;
                                continue;
                            }
                        }
                    }
                    return Err(e);
                }
            }
        }
    }

    /// Скачивает файл и сохраняет его на диск
    pub async fn download_file(&self, url: &str, path: &str) -> Result<()> {
        let response = self.fetch(url).await?;
        let content = response.bytes().await.map_err(|e| {
            AppErrors::Network(NetworkError::RequestFailed(format!(
                "ошибка получения байтов: {}",
                e
            )))
        })?;

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
        let response = self.fetch(url).await?;
        let text = response.text().await.map_err(|e| {
            AppErrors::Network(NetworkError::RequestFailed(format!(
                "ошибка получения текста: {}",
                e
            )))
        })?;

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
                Ok(response) => {
                    let text = response.text().await.map_err(|e| {
                        AppErrors::Network(NetworkError::RequestFailed(format!(
                            "ошибка получения текста: {}",
                            e
                        )))
                    })?;
                    results.push(text);
                }
                Err(e) => {
                    // Логируем ошибку и продолжаем
                    log::error!("Ошибка при загрузке {}: {}", url, e);
                    return Err(e);
                }
            }
        }

        Ok(results)
    }
}
