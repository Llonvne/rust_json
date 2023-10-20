use crate::number::JsonNumber;
use crate::object::JsonObject;
use crate::r#type::JsonType::{Array, Null, Number};

#[derive(Debug)]
pub enum JsonType<'a> {
    Null,
    True,
    False,
    Object(JsonObject<'a>),
    Array(Vec<JsonType<'a>>),
    String(String),
    Number(JsonNumber),
}

impl<'a> From<JsonNumber> for JsonType<'a> {
    fn from(value: JsonNumber) -> Self {
        Number(value)
    }
}

impl<'a, T> From<Vec<T>> for JsonType<'a>
where
    T: Into<JsonNumber>,
{
    fn from(value: Vec<T>) -> Self {
        Array(value.into_iter().map(|v| v.into()).map(Number).collect())
    }
}
