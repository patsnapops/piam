//! A parser module can be implemented to parse the request and return a [`Input`] struct.

pub trait Input: Sized + std::fmt::Debug {}
