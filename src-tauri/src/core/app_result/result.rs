use crate::core::app_result::ErrorDetails;

use super::{error::AppError, success::AppSuccess};

pub type AppResult<T> = Result<AppSuccess<T>, ErrorDetails>;
