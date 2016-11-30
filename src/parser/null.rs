use std::iter::Peekable;
use std::str::Chars;

use error::Error;
use json::Json;

use super::*;

pub fn null(slice: &mut Peekable<&mut Chars>) -> Result<Json, Error> {
    let s = "null";

    check_str(s, slice)?;

    Ok(Json::Null)
}
