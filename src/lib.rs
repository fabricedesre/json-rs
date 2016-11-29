#![cfg_attr(feature = "core", feature(collections))]
#![cfg_attr(feature = "core", no_std)]

#[cfg(feature = "core")]
#[macro_use]
extern crate collections;

#[cfg(feature = "core")]
mod std {
    pub use core::{char, fmt, iter, option, ops, slice, mem, str};
    pub use collections::{boxed, vec, string};

    pub mod prelude {
        pub use core::prelude as v1;
    }
}

#[cfg(feature = "core")]
mod hash_map;
mod error;
mod json;
mod number;
mod parser;

pub use self::error::Error;
pub use self::json::Json;
pub use self::number::Number;
