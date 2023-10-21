use crate::value::JsonValue;
use std::fmt::{Display, Formatter};
use std::slice::Iter;

#[derive(Debug)]
pub struct JsonArray<'a> {
    pub array: Vec<JsonValue<'a>>,
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

impl<'a> JsonArray<'a> {
    pub fn iter(&'a self) -> JsonArrayIter<'a> {
        let iter = self.array.iter();
        JsonArrayIter { iter }
    }
}

pub struct JsonArrayIter<'a> {
    iter: Iter<'a, JsonValue<'a>>,
}

impl<'a> Iterator for JsonArrayIter<'a> {
    type Item = &'a JsonValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
