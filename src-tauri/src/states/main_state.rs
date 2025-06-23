use crate::core::models::WallpaperHistory;
use std::sync::Arc;
use tauri::State;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Default)]
pub struct MainState {
    pub wallpaper_history: Arc<Mutex<Vec<WallpaperHistory>>>,
}

impl MainState {
    pub fn new() -> Self {
        Self {
            wallpaper_history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn get_wallpaper_history(&self) -> MutexGuard<'_, Vec<WallpaperHistory>> {
        self.wallpaper_history.lock().await
    }

    pub async fn add_wallpaper_history(&self, history: WallpaperHistory) {
        let mut history_lock = self.get_wallpaper_history().await;
        history_lock.push(history);
    }
}
