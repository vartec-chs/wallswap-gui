mod error;
mod error_categories;
mod result;
mod success;

pub use error::{AppError, ErrorDetails, ErrorSeverity};
pub use error_categories::*;
pub use result::AppResult;
pub use success::{AppSuccess, SuccessCode};
