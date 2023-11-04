use crate::array::JsonArray;
use crate::object::JsonObject;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum JsonValue<'a> {
    Number(Box<f64>),
    String(Box<&'a str>),
    Object(Box<JsonObject<'a>>),
    Array(Box<JsonArray<'a>>),
    True,
    False,
    Null,
    Empty,
}

impl<'a> JsonValue<'a> {
    pub fn try_as_array(&self) -> Option<&JsonArray> {
        match self {
            JsonValue::Array(array) => Some(array),
            _ => None,
        }
    }
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
            JsonValue::Empty => write!(f, ""),
        }
    }
}
