use neon::result::Throw;
use std::fmt;

#[cfg(not(tarpaulin_include))]
#[derive(fmt::Debug, Clone)]
pub struct IRError {
    error_type: IRErrorType,
    message: String,
}

#[cfg(not(tarpaulin_include))]
#[derive(fmt::Debug, Clone)]
pub enum IRErrorType {
    JsError,
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for IRErrorType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IRErrorType::JsError => write!(formatter, "JsErrorError"),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl IRError {
    pub fn new(message: String, error_type: IRErrorType) -> IRError {
        IRError {
            message,
            error_type,
        }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_error_type(&self) -> IRErrorType {
        self.error_type.clone()
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for IRError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "ir error {}, reason: {}",
            self.error_type, self.message
        )
    }
}

#[cfg(not(tarpaulin_include))]
impl From<Throw> for IRError {
    fn from(error: Throw) -> IRError {
        IRError::new(format!("{}", error), IRErrorType::JsError)
    }
}
