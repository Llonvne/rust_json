use crate::value::JsonValue;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct JsonArray<'a> {
    pub(crate) array: Vec<JsonValue<'a>>,
}

impl<'a> Display for JsonArray<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let len = self.array.len();
        for (index, value) in self.array.iter().enumerate() {
            write!(f, "{}", value)?;
            if index != len - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}
