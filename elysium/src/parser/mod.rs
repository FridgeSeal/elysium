//! Parsing module.

mod event;
mod grammar;
mod marker;
mod parser;
mod sink;
mod source;

pub use marker::CompletedMarker;
pub use parser::{parse, Parser};
