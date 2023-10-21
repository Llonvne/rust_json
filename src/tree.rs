use crate::token::JsonToken;
use crate::token::JsonToken::{Colon, RightBrace, String};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
#[derive(Debug)]
pub struct JsonObject<'a> {
    children: Vec<KeyValue<'a>>,
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

#[derive(Debug)]
pub struct KeyValue<'a> {
    key: &'a str,
    value: JsonValue<'a>,
}

impl<'a> Display for KeyValue<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.key)?;
        write!(f, ":")?;
        write!(f, "{}", self.value)
    }
}

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

#[derive(Debug)]
pub struct JsonArray<'a> {
    array: Vec<JsonValue<'a>>,
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
#[derive(Debug)]
pub struct JsonTokenIter<'a> {
    tokens: &'a Vec<JsonToken<'a>>,
    pos: RefCell<usize>,
}

impl<'a> JsonTokenIter<'a> {
    pub fn new(tokens: &'a Vec<JsonToken<'a>>) -> Rc<Self> {
        Rc::new(Self {
            tokens,
            pos: RefCell::new(0),
        })
    }

    pub fn parse(self: Rc<Self>) -> JsonObject<'a> {
        parse_object(Rc::clone(&self))
    }

    pub fn peek(&self) -> Option<&JsonToken<'a>> {
        self.tokens.get(*self.pos.borrow())
    }
    pub fn peek_offset(&self, offset: usize) -> Option<&JsonToken<'a>> {
        self.tokens.get(*self.pos.borrow() + offset)
    }

    pub fn next(&self) -> Option<&'a JsonToken<'a>> {
        let pos = {
            let pos = self.pos.borrow();
            *pos
        };
        *self.pos.borrow_mut() += 1;
        self.tokens.get(pos)
    }

    pub fn next_offset(&self, offset: usize) -> Option<&'a JsonToken<'a>> {
        let pos = {
            let pos = self.pos.borrow();
            *pos + offset
        };
        *self.pos.borrow_mut() = pos;
        self.tokens.get(pos)
    }
    pub fn last(&self) -> Option<&'a JsonToken<'a>> {
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
fn parse_object(tokens: Rc<JsonTokenIter>) -> JsonObject {
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

fn parse_value(tokens: Rc<JsonTokenIter>) -> JsonValue {
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
        _ => panic!("invaild json value"),
    };
    value
}

fn parse_array(iter: Rc<JsonTokenIter>) -> JsonArray {
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
