use crate::array::JsonArray;
use crate::error::JsonParserInternalError::TokenIndexOutOfRange;
use crate::error::{
    expect_a_comma_or_right_brace_after_value, expect_colon_after_key,
    expect_first_token_is_left_brace, expect_first_token_is_left_bracket_or_brace,
    expect_key_or_right_brace, JsonParserError, JsonParserInternalError, UnexpectedTokenErrorDecr,
};
use crate::keyvalue::KeyValue;
use crate::object::JsonObject;
use crate::parser::JsonParserError::{
    InternalJsonParserError, UnexpectedEndOfTokens, UnexpectedToken,
};
use crate::token::JsonToken::*;
use crate::token::{JsonToken, JsonTokenStream};
use crate::value::JsonValue;
use crate::value::JsonValue::{Array, Empty, Object};
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

    pub fn parse(self: Rc<Self>) -> Result<JsonValue<'a>, JsonParserError<'a>> {
        let first = self.tokens.first();
        match first {
            Some(first) => match first {
                LeftBrace => parse_object(Rc::clone(&self)).map(|obj| Object(Box::new(obj))),
                LeftBracket => parse_array(Rc::clone(&self)).map(|arr| Array(Box::new(arr))),
                token => Err(expect_first_token_is_left_bracket_or_brace(token))?,
            },
            None => Err(UnexpectedEndOfTokens)?,
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
    fn last(&self) -> Result<&'a JsonToken<'a>, JsonParserInternalError> {
        let pos = {
            let pos = self.pos.borrow();
            (*pos as isize) - 1
        };
        if pos < 0 {
            return Err(TokenIndexOutOfRange);
        }
        *self.pos.borrow_mut() = pos as usize;
        Ok(self.tokens.get(pos as usize).unwrap())
    }
}

fn parse_object(tokens: Rc<Parser>) -> Result<JsonObject, JsonParserError> {
    let mut obj = JsonObject { children: vec![] };
    match tokens.next() {
        Some(LeftBrace) => {}
        Some(token) => Err(expect_first_token_is_left_brace(token))?,
        None => Err(UnexpectedEndOfTokens)?,
    }
    loop {
        match tokens.next() {
            Some(RightBrace) => {
                break;
            }
            Some(String(key)) => {
                let colon_token = tokens.next();
                if colon_token != Some(&Colon) {
                    match colon_token {
                        None => Err(UnexpectedEndOfTokens)?,
                        Some(token) => Err(expect_colon_after_key(token))?,
                    };
                } else {
                    obj.children.push(KeyValue {
                        key,
                        value: parse_value(Rc::clone(&tokens))?,
                    });
                }
                let next = tokens.next();
                match next {
                    None => Err(UnexpectedEndOfTokens)?,
                    Some(Comma) => {}
                    Some(RightBrace) => {
                        break;
                    }
                    Some(token) => Err(expect_a_comma_or_right_brace_after_value(token))?,
                }
            }
            Some(token) => Err(expect_key_or_right_brace(token))?,
            None => Err(UnexpectedEndOfTokens)?,
        }
    }

    Ok(obj)
}

fn parse_value(tokens: Rc<Parser>) -> Result<JsonValue, JsonParserError> {
    match tokens.next() {
        None => Err(UnexpectedEndOfTokens)?,
        Some(token) => match token {
            String(str) => Ok(JsonValue::String(Box::new(str))),
            Number(num) => Ok(JsonValue::Number(Box::new(*num))),
            True => Ok(JsonValue::True),
            False => Ok(JsonValue::False),
            Null => Ok(JsonValue::Null),
            LeftBrace => {
                tokens.last().map_err(InternalJsonParserError)?;
                parse_object(Rc::clone(&tokens)).map(|obj| Object(Box::new(obj)))
            }
            LeftBracket => {
                tokens.last().map_err(InternalJsonParserError)?;
                parse_array(Rc::clone(&tokens)).map(|arr| Array(Box::new(arr)))
            }
            RightBracket => {
                tokens.last().map_err(InternalJsonParserError)?;
                Ok(Empty)
            }
            RightBrace => {
                tokens.last().map_err(InternalJsonParserError)?;
                Ok(Empty)
            }
            _ => Err(UnexpectedToken(UnexpectedTokenErrorDecr {
                expect: "string, number, true, false, null, {, [",
                actual: token,
                msg: "it should be string, number, true, false, null, {, [",
            })),
        },
    }
}

fn parse_array(iter: Rc<Parser>) -> Result<JsonArray, JsonParserError> {
    let mut arr = JsonArray { array: vec![] };
    loop {
        match iter.next() {
            None => Err(UnexpectedEndOfTokens)?,
            Some(token) => match token {
                RightBracket => break,
                _token => parse_value(Rc::clone(&iter)).map(|value| {
                    arr.array.push(value);
                })?,
            },
        }
    }
    Ok(arr)
}
