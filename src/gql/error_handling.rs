use async_graphql::{Error, ErrorExtensions};
pub fn append_err(errors: Option<Error>, name: &'static str, value: &'static str) -> Option<Error> {
    Some(errors.unwrap().extend_with(|_, e| e.set(name, value)))
}