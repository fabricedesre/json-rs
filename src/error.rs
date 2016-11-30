#[cfg(feature = "core")]
use collections::String;

pub enum Error {
    UnexpectedEof,
    InvalidCharacter(String),
}
