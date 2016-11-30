#[cfg(feature = "core")]
pub use collections::{String, Vec};

#[cfg(not(feature = "core"))]
use std::collections::HashMap;
#[cfg(feature = "core")]
use hash_map::HashMap;

use error::Error;
use parser::node;
use number::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Json {
    Object(HashMap<String, Json>),
    Array(Vec<Json>),
    String(String),
    Number(Number),
    Boolean(bool),
    Null,
}

impl Json {
    pub fn parse(text: &str) -> Result<Json, Error> {
        let mut slice = text.chars();
        let mut peekable = (&mut slice).peekable();

        node(&mut peekable)
    }

    #[cfg(not(feature = "core"))]
    pub fn to_string(&self) -> String {
        let mut string: String = String::new();

        match self {
            &Json::Null => {
                string.push_str("null");
            }
            &Json::Boolean(value) => {
                string.push_str(if value { "true" } else { "false" });
            }
            &Json::Number(ref value) => {
                string.push_str(value.to_string().as_str());
            }
            &Json::String(ref value) => {
                string.push('"');
                for chr in value.chars() {
                    if chr == '"' {
                        string.push('\\');
                    }
                    string.push(chr);
                }
                string.push('"');
            }
            &Json::Array(ref value) => {
                let mut first = true;

                string.push('[');
                for elem in value {
                    if !first {
                        string.push(',');
                    }
                    string.push_str(elem.to_string().as_str());
                    first = false;
                }
                string.push(']');
            }
            &Json::Object(ref value) => {
                let mut first = true;

                string.push('{');
                for (k, v) in value {
                    if !first {
                        string.push(',');
                    }
                    string.push('"');
                    for chr in k.chars() {
                        if chr == '"' {
                            string.push('\\');
                        }
                        string.push(chr);
                    }
                    string.push('"');
                    string.push(':');
                    string.push_str(v.to_string().as_str());
                    first = false;
                }
                string.push('}');
            }
        }

        string
    }
}
