#[cfg(feature = "core")]
use collections::string::ToString;
use std::iter::Peekable;
use std::str::Chars;

use error::Error;

pub fn check_str(s: &str, slice: &mut Peekable<&mut Chars>) -> Result<(), Error> {
    for c in s.chars() {
        let current = match slice.next() {
            Some(chr) => chr,
            None => {
                return Err(Error::UnexpectedEof);
            }
        };

        if current != c {
            return Err(Error::InvalidCharacter(current.to_string()));
        }
    }
    Ok(())
}

mod node;
mod null;
mod boolean;
mod number;
mod string;
mod array;
mod object;

pub use self::node::node;
pub use self::null::null;
pub use self::boolean::boolean;
pub use self::number::number;
pub use self::string::string;
pub use self::array::array;
pub use self::object::object;
