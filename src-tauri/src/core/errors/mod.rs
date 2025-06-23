mod app_errors;
mod categories;
mod converters;

pub use app_errors::*;
pub use categories::*;
pub use converters::*;

pub type Result<T> = std::result::Result<T, AppErrors>;
