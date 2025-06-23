use crate::core::models::{Category, ResolutionFilter};
use std::sync::Arc;
use tauri::State;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Default)]
pub struct FilterState {
    pub categories: Arc<Mutex<Vec<Category>>>,
    pub resolutions: Arc<Mutex<Vec<ResolutionFilter>>>,
}

impl FilterState {
    fn new() -> Self {
        Self {
            categories: Arc::new(Mutex::new(Vec::new())),
            resolutions: Arc::new(Mutex::new(Vec::new())),
        }
    }

    async fn get_categories(&self) -> MutexGuard<'_, Vec<Category>> {
        self.categories.lock().await
    }

    async fn get_resolutions(&self) -> MutexGuard<'_, Vec<ResolutionFilter>> {
        self.resolutions.lock().await
    }
}
