use screen_size::get_primary_screen_size;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Категория обоев
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub name: String,
    pub count_wallpapers: Option<u64>,
    pub url: String,
}

impl Category {
    pub fn new(name: String, url: String, count_wallpapers: Option<u64>) -> Self {
        Self {
            name,
            url,
            count_wallpapers,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResolutionFilter {
    pub name: String,
    pub category: Option<String>, // Категория, к которой относится разрешение
    pub resolution: String,
    pub url: String,
}
impl ResolutionFilter {
    pub fn new(name: String, resolution: String, url: String, category: Option<String>) -> Self {
        Self {
            name,
            resolution,
            url,
            category,
        }
    }
}

/// Запись в истории обоев
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallpaperHistory {
    pub id: String,
    pub url: String,
    pub local_path: String,
    pub category: String,
    pub timestamp: SystemTime,
}

impl WallpaperHistory {
    pub fn new(url: String, local_path: String, category: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            url,
            local_path,
            category,
            timestamp: SystemTime::now(),
        }
    }
}

/// Информация об обоях
#[derive(Debug, Clone)]
pub struct WallpaperInfo {
    pub rating: Option<String>,
    pub votes: Option<i32>,
    pub resolution: Option<String>,
    pub downloads: Option<i32>,
    pub uploaded: Option<String>,
    pub title: Option<String>,
    pub tags: Option<String>,
    pub download_url: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
}

impl WallpaperInfo {
    pub fn new(
        rating: Option<String>,
        votes: Option<i32>,
        resolution: Option<String>,
        downloads: Option<i32>,
        uploaded: Option<String>,
        title: Option<String>,
        tags: Option<String>,
        download_url: Option<String>,
        author: Option<String>,
        license: Option<String>,
    ) -> Self {
        Self {
            rating,
            votes,
            resolution,
            downloads,
            uploaded,
            title,
            tags,
            download_url,
            author,
            license,
        }
    }
}

/// Конфигурация приложения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub download_quality: WallpaperQuality,
    pub cache_size_limit: u64, // В байтах
    pub cache_count: u64,
    pub auto_cleanup: bool,
    pub user_agent: String,
    pub request_timeout: u64, // В секундах
}

impl Default for AppConfig {
    fn default() -> Self {
        let (width, height) = match get_primary_screen_size() {
            Ok(size) => (size.0, size.1),
            Err(_) => (1920, 1080), // Значение по умолчанию, если не удалось получить размер экрана
        };

        // Проверка на минимальный размер экрана
        if width < 1366 || height < 768 {
            eprintln!("Warning: Screen resolution is below the recommended minimum of 1366x768.");
        }

        let default_quality = if width >= 3840 && height >= 2160 {
            WallpaperQuality::Ultra
        } else if width >= 2560 && height >= 1440 {
            WallpaperQuality::High
        } else if width >= 1920 && height >= 1080 {
            WallpaperQuality::Medium
        } else {
            WallpaperQuality::Low
        };

        Self {
            download_quality: default_quality,
            cache_size_limit: 1024 * 1024 * 1024, // 1GB
            auto_cleanup: true,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36".to_string(),
            request_timeout: 30,
			cache_count: 100, // Максимальное количество кэшированных обоев
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WallpaperQuality {
    Low,    // 1366x768
    Medium, // 1920x1080
    High,   // 2560x1440
    Ultra,  // 3840x2160
}

impl WallpaperQuality {
    pub fn to_resolution(&self) -> &'static str {
        match self {
            WallpaperQuality::Low => "1366x768",
            WallpaperQuality::Medium => "1920x1080",
            WallpaperQuality::High => "2560x1440",
            WallpaperQuality::Ultra => "3840x2160",
        }
    }
}
