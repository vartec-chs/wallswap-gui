pub mod errors;
mod fetcher;
pub mod models;
mod path;
mod set_wallpaper;
mod types;
mod utils;

pub use fetcher::HttpFetcher;
pub use utils::api_result;
pub use utils::debug;
