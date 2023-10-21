use crate::array::JsonArray;
use crate::keyvalue::KeyValue;
use crate::object::JsonObject;
use crate::token::JsonToken::{Colon, LeftBrace, RightBrace, String};
use crate::token::{JsonToken, JsonTokenStream};
use crate::value::JsonValue;
use crate::value::JsonValue::{Array, Object};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<JsonToken<'a>>,
    pos: RefCell<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a JsonTokenStream) -> Rc<Self> {
        let tokens = &tokens.tokens;
        Rc::new(Self {
            tokens,
            pos: RefCell::new(0),
        })
    }

    pub fn parse(self: Rc<Self>) -> JsonValue<'a> {
        let first = self.tokens.first().expect("tokens should be empty");
        match first {
            LeftBrace => Object(Box::new(parse_object(Rc::clone(&self)))),
            JsonToken::LeftBracket => Array(Box::new(parse_array(Rc::clone(&self)))),
            _ => panic!("it should be [ or }}"),
        }
    }

    fn peek(&self) -> Option<&JsonToken<'a>> {
        self.tokens.get(*self.pos.borrow())
    }
    fn peek_offset(&self, offset: usize) -> Option<&JsonToken<'a>> {
        self.tokens.get(*self.pos.borrow() + offset)
    }

    fn next(&self) -> Option<&'a JsonToken<'a>> {
        let pos = {
            let pos = self.pos.borrow();
            *pos
        };
        *self.pos.borrow_mut() += 1;
        self.tokens.get(pos)
    }

    fn next_offset(&self, offset: usize) -> Option<&'a JsonToken<'a>> {
        let pos = {
            let pos = self.pos.borrow();
            *pos + offset
        };
        *self.pos.borrow_mut() = pos;
        self.tokens.get(pos)
    }
    fn last(&self) -> Option<&'a JsonToken<'a>> {
        let pos = {
            let pos = self.pos.borrow();
            (*pos as isize) - 1
        };
        if pos < 0 {
            panic!("pos is 0 cannot get last")
        }
        *self.pos.borrow_mut() = pos as usize;
        self.tokens.get(pos as usize)
    }
}
fn parse_object(tokens: Rc<Parser>) -> JsonObject {
    let mut obj = JsonObject { children: vec![] };
    match tokens.next() {
        Some(JsonToken::LeftBrace) => {}
        Some(token) => {
            dbg!(token);
            panic!("it should be {{")
        }
        None => panic!("tokens should not be empty"),
    }

    loop {
        match tokens.next() {
            Some(RightBrace) => break,
            Some(String(key)) => {
                let colon_token = tokens.next();
                if colon_token != Some(&Colon) {
                    panic!("Expected ':' after key");
                } else {
                    obj.children.push(KeyValue {
                        key,
                        value: parse_value(Rc::clone(&tokens)),
                    });
                }
                let next = tokens.next().expect("it should be empty after key value");
                match next {
                    JsonToken::Comma => {}
                    RightBrace => break,
                    _ => {
                        dbg!(next);
                        panic!("Unexpected token");
                    }
                }
            }
            Some(token) => {
                dbg!(token);
                panic!("Unexpected token")
            }
            None => panic!("Unexpected end of tokens"),
        }
    }

    obj
}

fn parse_value(tokens: Rc<Parser>) -> JsonValue {
    let value = match tokens.next().expect("") {
        String(str) => JsonValue::String(Box::new(str)),
        JsonToken::Number(num) => JsonValue::Number(Box::new(*num)),
        JsonToken::True => JsonValue::True,
        JsonToken::False => JsonValue::False,
        JsonToken::Null => JsonValue::Null,
        JsonToken::LeftBrace => {
            tokens.last();
            JsonValue::Object(Box::new(parse_object(tokens)))
        }
        JsonToken::LeftBracket => {
            tokens.last();
            JsonValue::Array(Box::new(parse_array(tokens)))
        }
        _ => panic!("invalid json value"),
    };
    value
}

fn parse_array(iter: Rc<Parser>) -> JsonArray {
    let mut arr = JsonArray { array: vec![] };

    loop {
        match iter.next() {
            None => panic!("it should be None"),
            Some(token) => match token {
                JsonToken::RightBracket => break,
                _ => arr.array.push(parse_value(Rc::clone(&iter))),
            },
        }
    }
    arr
}
