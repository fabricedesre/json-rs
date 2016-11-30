#[cfg(feature = "core")]
use collections::string::ToString;

use std::iter::Peekable;
use std::str::Chars;

use error::Error;
use json::Json;

use super::*;

pub fn boolean(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error> {
    let value;

    let s = {
        let current = match slice.peek() {
            Some(chr) => *chr,
            None => {
                return Err(Error::UnexpectedEof);
            }
        };

        match current {
            'f' => {
                value = false;
                "false"
            }
            't' => {
                value = true;
                "true"
            }
            _ => {
                // This can't happen...
                return Err(Error::InvalidCharacter(current.to_string()));
            }
        }
    };

    check_str(s, slice)?;

    Ok(Json::Boolean(value))
}
