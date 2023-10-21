use crate::token::JsonToken;
use crate::token::JsonToken::{Colon, RightBrace, String};
use std::ops::{Add, AddAssign};
use std::slice::Iter;

pub struct JsonObject<'a> {
    children: Vec<KeyValue<'a>>,
}

pub struct KeyValue<'a> {
    key: &'a str,
    value: JsonValue<'a>,
}

pub enum JsonValue<'a> {
    Number(i64),
    String(&'a str),
    Object(JsonObject<'a>),
    Array(JsonArray),
    True,
    False,
    Null,
}

pub enum JsonArray {
    I64Array(Vec<i64>),
}
#[derive(Debug)]
pub struct JsonTokenIter<'a> {
    tokens: &'a Vec<JsonToken<'a>>,
    pos: usize,
}

impl<'a> JsonTokenIter<'a> {
    pub fn new(tokens: &'a Vec<JsonToken<'a>>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> JsonObject {
        parse_object(self)
    }

    pub fn peek(&self) -> Option<&JsonToken<'a>> {
        self.tokens.get(self.pos)
    }
    pub fn peek_offset(&self, offset: usize) -> Option<&JsonToken<'a>> {
        self.tokens.get(self.pos.add(offset))
    }

    pub fn next(&mut self) -> Option<&JsonToken<'a>> {
        let cur = self.peek();
        unsafe {
            let pos = &mut *(self.pos as *mut usize);
            *pos += 1;
        }
        cur
    }

    pub fn next_offset(&mut self, offset: usize) -> Option<&JsonToken<'a>> {
        self.pos.add_assign(1);
        self.peek()
    }
}
fn parse_object<'a>(tokens: &mut JsonTokenIter<'a>) -> JsonObject<'a> {
    let mut obj = JsonObject { children: vec![] };
    match tokens.next() {
        Some(JsonToken::LeftBrace) => {}
        Some(_) => panic!("tokens should be {{"),
        None => panic!("tokens should not be empty"),
    }

    loop {
        match tokens.peek() {
            Some(RightBrace) => break,
            Some(String(key)) => {
                let colon_token = tokens.peek_offset(1);
                if colon_token != Some(&Colon) {
                    panic!("Expected ':' after key");
                } else {
                    obj.children.push(KeyValue {
                        key,
                        value: parse_value(tokens),
                    });
                    tokens.next_offset(2);
                }
            }
            Some(_) => panic!("Unexpected token"),
            None => panic!("Unexpected end of tokens"),
        }
    }

    obj
}

fn parse_value<'a>(tokens: &mut JsonTokenIter<'a>) -> JsonValue<'a> {
    let value = match tokens.next().expect("") {
        String(str) => JsonValue::String(str),
        JsonToken::Number(num) => JsonValue::Number(*num),
        JsonToken::True => JsonValue::True,
        JsonToken::False => JsonValue::False,
        JsonToken::Null => JsonValue::Null,
        JsonToken::LeftBrace => JsonValue::Object(parse_object(tokens)),
        JsonToken::LeftBracket => JsonValue::Array(parse_array(tokens)),
        _ => panic!("invaild json value"),
    };
    value
}

fn parse_array(iter: &JsonTokenIter) -> JsonArray {
    JsonArray::I64Array(vec![1, 2, 3])
}
