use std::env;
use tauri::path::BaseDirectory::AppCache;
use tauri::path::BaseDirectory::AppData;
use tokio::fs;

use crate::core::errors::{AppErrors, FileSystemError, Result};
use std::path::PathBuf;

pub fn get_home_dir() -> Result<PathBuf> {
    match env::var(AppData.variable()) {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_) => Err(AppErrors::FileSystem(FileSystemError::IoError(
            "Не удалось получить домашнюю директорию".to_string(),
        ))),
    }
}
pub async fn get_cache_dir() -> Result<PathBuf> {
    let path = env::var(AppCache.variable()).map_err(|_| {
        AppErrors::FileSystem(FileSystemError::IoError(
            "Не удалось получить кэш директорию".to_string(),
        ))
    })?;
    let cache_path = PathBuf::from(path);
    let wallswap_path = cache_path.join("WallSwap");
    if !wallswap_path.exists() {
        fs::create_dir_all(&wallswap_path)
            .await
            .map_err(|e| AppErrors::FileSystem(FileSystemError::IoError(e.to_string())))?;
    }
    log::info!("Кэш директория: {}", wallswap_path.display());
    Ok(wallswap_path)
}
pub async fn debug_dir() -> Result<PathBuf> {
	let path = get_cache_dir().await?;
	let wallswap_path = path.join("debug_output");
	if !wallswap_path.exists() {
		fs::create_dir_all(&wallswap_path)
			.await
			.map_err(|e| AppErrors::FileSystem(FileSystemError::IoError(e.to_string())))?;
	}
	log::info!("Директория отладки: {}", wallswap_path.display());
	Ok(wallswap_path)
}

pub async fn download_dir() -> Result<PathBuf> {
    let path = get_cache_dir().await?;
    let wallswap_path = path.join("downloads");
    if !wallswap_path.exists() {
        fs::create_dir_all(&wallswap_path)
            .await
            .map_err(|e| AppErrors::FileSystem(FileSystemError::IoError(e.to_string())))?;
    }
    log::info!("Директория загрузок: {}", wallswap_path.display());
    Ok(wallswap_path)
}

pub async fn get_data_dir() -> Result<PathBuf> {
    let path = env::var(AppData.variable()).map_err(|_| {
        AppErrors::FileSystem(FileSystemError::IoError(
            "Не удалось получить директорию данных".to_string(),
        ))
    })?;
    let data_path = PathBuf::from(path);
    let wallswap_path = data_path.join("WallSwap");
    if !wallswap_path.exists() {
        fs::create_dir_all(&wallswap_path)
            .await
            .map_err(|e| AppErrors::FileSystem(FileSystemError::IoError(e.to_string())))?;
    }
    log::info!("Директория данных: {}", wallswap_path.display());
    Ok(wallswap_path)
}
