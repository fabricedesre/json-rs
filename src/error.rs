#[cfg(feature = "core")]
use collections::String;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
    InvalidCharacter(String),
}
