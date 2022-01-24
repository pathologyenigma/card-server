pub use async_graphql::{Error, ErrorExtensions};
pub trait ErrorHandler {
    type ErrorType;
    fn append(&mut self, name: &'static str, value: &'static str);
    fn to_err(self) -> Self::ErrorType;
}

#[derive(Clone)]
pub struct BadInputErrorHandler {
    errors: Option<Error>,
}

impl ErrorHandler for BadInputErrorHandler {
    type ErrorType = Error;

    fn append(&mut self, name: &'static str, value: &'static str) {
        self.errors = Some(self.errors.clone().unwrap_or(Error::new("Bad Input")).extend_with(|_, e| e.set(name, value)));
    }

    fn to_err(self) -> Self::ErrorType {
        self.errors.unwrap()
    }
}

impl Default for BadInputErrorHandler {
    fn default() -> Self {
        Self {
            errors: None,
        }
    }
}

impl BadInputErrorHandler {
    pub fn is_none(&self) -> bool {
        self.errors.is_none()
    }
}