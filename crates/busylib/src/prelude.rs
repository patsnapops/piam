use std::fmt::Display;

use log::error;

/// eok stands for it is expected to be [`Ok`], unexpected [`Err`] will be logged.
pub fn eok<T, E: Display>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            error!("this should never happen: {}", e);
            panic!("this should never happen: {}", e);
        }
    }
}

/// esome stands for it is expected to be [`Some`], unexpected [`None`] will be logged.
pub fn esome<T>(option: Option<T>) -> T {
    match option {
        Some(value) => value,
        None => {
            error!("this should never happen: None");
            panic!("this should never happen: None");
        }
    }
}

/// eok with custom context
pub fn eok_ctx<T, E: Display>(result: Result<T, E>, msg: &str) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            error!("this should never happen: {}, context: {}", e, msg);
            panic!("this should never happen: {}, context: {}", e, msg);
        }
    }
}

/// esome with custom context
pub fn esome_ctx<T>(option: Option<T>, msg: &str) -> T {
    match option {
        Some(value) => value,
        None => {
            error!("this should never happen: None, context: {}", msg);
            panic!("this should never happen: None, context: {}", msg);
        }
    }
}
