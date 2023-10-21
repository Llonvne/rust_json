use crate::object::JsonObject;
use crate::parser::Parser;
use crate::token::{parse_to_tokens, JsonTokenStream};

pub mod array;
pub mod keyvalue;
pub mod object;
pub mod parser;
pub mod token;
pub mod value;

pub struct JsonObjectWrapper<'a> {
    pub object: JsonObject<'a>,
    origin: &'a str,
}

impl<'a> From<&'a str> for JsonObjectWrapper<'a> {
    fn from(value: &'a str) -> Self {
        let tokens = parse_to_tokens(value).expect("");
        let obj = Parser::new(&tokens).parse();
        JsonObjectWrapper {
            object: obj,
            origin: value,
        }
    }
}
