//! A parser module can be implemented to parse the request and return a [`Input`] struct.

use crate::type_alias::HttpRequest;

pub trait Input: Sized + std::fmt::Debug {}

pub struct InputAndRequest<T> {
    input: T,
    request: HttpRequest,
}

impl<T> InputAndRequest<T> {
    pub fn new(input: T, request: HttpRequest) -> Self {
        Self { input, request }
    }

    pub fn into_parts(self) -> (T, HttpRequest) {
        (self.input, self.request)
    }
}
