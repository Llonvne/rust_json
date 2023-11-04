use crate::keyvalue::KeyValue;
use crate::value::JsonValue;
use std::fmt::{Display, Formatter};
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub struct JsonObject<'a> {
    pub(crate) children: Vec<KeyValue<'a>>,
}

impl<'a> JsonObject<'a> {
    pub fn iter(&self) -> JsonObjectIter {
        let iter = self.children.iter();
        JsonObjectIter { iter }
    }

    pub fn get_by_key(&self, str: &str) -> Option<&'a JsonValue> {
        let candidate: Vec<&KeyValue> = self
            .children
            .iter()
            .filter(|KeyValue { key, .. }| str == *key)
            .collect();
        let value = candidate.get(0);
        value.map(|KeyValue { key: _, value }| value)
    }
}

impl<'a> Display for JsonObject<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let len = self.children.len();
        for (index, child) in self.children.iter().enumerate() {
            write!(f, "{}", child)?;
            if index != len - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")
    }
}

impl<'a> Iterator for JsonObjectIter<'a> {
    type Item = &'a KeyValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct JsonObjectIter<'a> {
    iter: Iter<'a, KeyValue<'a>>,
}
