use crate::core::models::{Category, ResolutionFilter};
use std::sync::Arc;

use tokio::sync::{Mutex, MutexGuard};

#[derive(Default)]
pub struct FilterState {
    pub categories: Arc<Mutex<Vec<Category>>>,
    pub resolutions: Arc<Mutex<Vec<ResolutionFilter>>>,
    pub selected_category_index: Arc<Mutex<Option<usize>>>,
    pub selected_resolution_index: Arc<Mutex<Option<usize>>>,
}

impl FilterState {
    fn new() -> Self {
        Self {
            categories: Arc::new(Mutex::new(Vec::new())),
            resolutions: Arc::new(Mutex::new(Vec::new())),
            selected_category_index: Arc::new(Mutex::new(None)),
            selected_resolution_index: Arc::new(Mutex::new(None)),
        }
    }

    async fn get_categories(&self) -> MutexGuard<'_, Vec<Category>> {
        self.categories.lock().await
    }

    async fn get_resolutions(&self) -> MutexGuard<'_, Vec<ResolutionFilter>> {
        self.resolutions.lock().await
    }

    async fn get_selected_category_index(&self) -> MutexGuard<'_, Option<usize>> {
        self.selected_category_index.lock().await
    }

    async fn get_selected_resolution_index(&self) -> MutexGuard<'_, Option<usize>> {
        self.selected_resolution_index.lock().await
    }

    // Example methods to manipulate the state
    pub async fn set_categories(&self, categories: Vec<Category>) {
        let mut guard = self.get_categories().await;
        *guard = categories;
    }

    pub async fn set_resolutions(&self, resolutions: Vec<ResolutionFilter>) {
        let mut guard = self.get_resolutions().await;
        *guard = resolutions;
    }

    pub async fn set_selected_category_index(&self, index: Option<usize>) {
        let mut guard = self.get_selected_category_index().await;
        *guard = index;
    }

    pub async fn set_selected_resolution_index(&self, index: Option<usize>) {
        let mut guard = self.get_selected_resolution_index().await;
        *guard = index;
    }

    // pub fn with_state<'b, F, R>(state: &'b mut State<'b, FilterState<'a>>, f: F) -> R
    // where
    //     F: FnOnce(&'b FilterState<'a>) -> R,
    // {
    //     f(state)
    // }

    // pub fn with_state_mut<'b, F, R>(state: &'b mut State<'b, FilterState<'a>>, f: F) -> R
    // where
    //     F: FnOnce(&'b mut FilterState<'a>) -> R,
    // {
    //     f(state)
    // }
}
