use crate::array::JsonArray;
use crate::object::JsonObject;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum JsonValue<'a> {
    Number(Box<i64>),
    String(Box<&'a str>),
    Object(Box<JsonObject<'a>>),
    Array(Box<JsonArray<'a>>),
    True,
    False,
    Null,
}

impl<'a> Display for JsonValue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::Number(number) => write!(f, "{}", number),
            JsonValue::String(str) => write!(f, "\"{}\"", str),
            JsonValue::Object(obj) => write!(f, "{}", obj),
            JsonValue::Array(arr) => write!(f, "{}", arr),
            JsonValue::True => write!(f, "true"),
            JsonValue::False => write!(f, "false"),
            JsonValue::Null => write!(f, "null"),
        }
    }
}
