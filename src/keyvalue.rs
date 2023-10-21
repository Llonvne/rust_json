use crate::value::JsonValue;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct KeyValue<'a> {
    pub key: &'a str,
    pub value: JsonValue<'a>,
}

impl<'a> Display for KeyValue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.key)?;
        write!(f, ":")?;
        write!(f, "{}", self.value)
    }
}
