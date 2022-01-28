pub use async_graphql::{Error, ErrorExtensions};
pub trait ErrorHandlerWithErrorExtensions {
    type ErrorType;
    fn append(&mut self, name: String, value: String);
    fn to_err(self) -> Self::ErrorType;
}

#[derive(Clone)]
pub struct BadInputErrorHandler {
    errors: Option<Error>,
}

impl ErrorHandlerWithErrorExtensions for BadInputErrorHandler {
    type ErrorType = Error;

    fn append(&mut self, name: String, value: String) {
        self.errors = Some(
            self.errors
                .clone()
                .unwrap_or(Error::new("400 Bad Input"))
                .extend_with(|_, e| e.set(name, value)),
        );
    }

    fn to_err(self) -> Self::ErrorType {
        self.errors.unwrap()
    }
}

impl Default for BadInputErrorHandler {
    fn default() -> Self {
        Self { errors: None }
    }
}

impl BadInputErrorHandler {
    pub fn is_none(&self) -> bool {
        self.errors.is_none()
    }
}
pub fn new_not_authenticated_error(msg: String) -> Error {
    Error::new("401 Not Authenticated").extend_with(|_, e| e.set("token", msg))
}

pub fn new_internal_server_error(msg: String) -> Error {
    Error::new("500 Internal Server Error").extend_with(|_, e| e.set("err_msg", msg))
}
