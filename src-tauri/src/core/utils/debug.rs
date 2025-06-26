use crate::core::api_result::{AppErrors, Result};
use crate::core::path::debug_dir;
use log;
use tokio::fs;

pub struct Debug;

impl Debug {
    /// Сохраняет HTML в файл для отладки
    pub async fn save_html_debug(html: &str, filename: &str) -> Result<()> {
        // Получаем директорию приложения
        let debug_dir = debug_dir().await?;

        let file_path = debug_dir.join(filename);
        fs::write(&file_path, html)
            .await
            .map_err(|e| AppErrors::FileSystem(e.into()))?;
        log::info!("🐛 HTML сохранен в: {}", file_path.display());
        Ok(())
    }

    /// Выводит первые несколько строк HTML
    pub fn preview_html(html: &str, lines: usize) {
        log::info!("📄 Превью HTML (первые {} строк):", lines);
        for (i, line) in html.lines().take(lines).enumerate() {
            log::info!(
                "{:3}: {}",
                i + 1,
                line.chars().take(100).collect::<String>()
            );
        }
    }
}
