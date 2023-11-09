use crate::token::TokenParseError::{KeyParseError, NumberParseError};
use std::num::ParseFloatError;
use std::str::CharIndices;
use JsonToken::*;

#[derive(Debug)]
pub struct JsonTokenStream<'a> {
    pub(crate) tokens: Vec<JsonToken<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum JsonToken<'a> {
    LeftBrace,
    RightBrace,
    DoubleQuote,
    Comma,
    Number(f64),
    Colon,
    True,
    False,
    Null,
    String(&'a str),
    LeftBracket,
    RightBracket,
}
#[derive(Debug, PartialEq)]
pub enum TokenParseError {
    KeyParseError,
    NumberParseError(NumberParseErrorKind),
}

#[derive(Debug, PartialEq)]
pub enum NumberParseErrorKind {
    DoubleDotInNumber,
    NumberParseError(ParseFloatError),
}

pub fn parse_to_tokens(origin: &str) -> Result<JsonTokenStream, TokenParseError> {
    let mut tokens: Vec<JsonToken> = Vec::new();
    let mut char_indices = origin.char_indices();

    while let Some((index, char)) = char_indices.next() {
        skip_whitespace(&mut char_indices);
        match char {
            '"' => {
                let (str, end) = parse_key(char_indices.clone().as_str())?;
                char_indices.nth(end);
                tokens.push(String(str));
            }
            // number
            '0'..='9' | '+' | '-' => {
                let (number, _) = parse_number(&mut char_indices, index, origin)?;
                tokens.push(Number(number));
            }
            // 匹配 true false null
            't' | 'f' | 'n' => {
                if parse_const_if_ok_then_skip(
                    char_indices.clone().as_str(),
                    "rue",
                    &mut char_indices,
                )
                .is_some()
                {
                    tokens.push(True);
                }
                if parse_const_if_ok_then_skip(
                    char_indices.clone().as_str(),
                    "alse",
                    &mut char_indices,
                )
                .is_some()
                {
                    tokens.push(False);
                }
                if parse_const_if_ok_then_skip(
                    char_indices.clone().as_str(),
                    "ull",
                    &mut char_indices,
                )
                .is_some()
                {
                    tokens.push(JsonToken::Null);
                }
            }
            '{' => tokens.push(LeftBrace),
            '}' => tokens.push(RightBrace),
            '[' => tokens.push(LeftBracket),
            ']' => tokens.push(RightBracket),
            ',' => tokens.push(Comma),
            ':' => tokens.push(Colon),
            _ => {}
        }
    }
    Ok(JsonTokenStream { tokens })
}

fn parse_number(
    char_indices: &mut CharIndices<'_>,
    current_index: usize,
    origin: &str,
) -> Result<(f64, usize), TokenParseError> {
    let peek = char_indices.clone().peekable();
    let start = current_index;
    let mut end = current_index;
    let mut has_dot = false;

    for (_index, char) in peek {
        if char == '.' {
            if has_dot {
                return Err(NumberParseError(NumberParseErrorKind::DoubleDotInNumber));
            } else {
                has_dot = true;
            }
        }

        if char.is_ascii_digit() || char == '.' {
            end = _index;
        } else {
            break;
        }
    }

    // Sync the state to char_indices
    if end - start > 0 {
        char_indices.nth(end - start - 1);
    }

    let number_str = &origin[start..end + 1];
    match number_str.parse::<f64>() {
        Ok(number) => Ok((number, end)),
        Err(e) => Err(NumberParseError(NumberParseErrorKind::NumberParseError(e))),
    }
}

fn parse_key(input: &str) -> Result<(&str, usize), TokenParseError> {
    let char_indices = input.char_indices();
    let start_index = 0;
    let mut end_index = None;
    let mut ignore_next = false;

    for (index, char) in char_indices {
        if ignore_next {
            ignore_next = false;
            end_index = Some(index);
            continue;
        }

        match char {
            '\\' => {
                ignore_next = true;
            }
            '"' => {
                // This is the closing quote of the key
                end_index = Some(index);
                break;
            }
            _ => continue,
        }
    }

    match end_index {
        Some(end) => Ok((&input[start_index..end], end)),
        _ => Err(KeyParseError),
    }
}

fn parse_const_if_ok_then_skip(input: &str, pattern: &str, chars: &mut CharIndices) -> Option<()> {
    match parse_const(input, pattern) {
        Some(size) => {
            chars.nth(size - 1);
            Some(())
        }
        None => None,
    }
}

fn parse_const(input: &str, const_str: &str) -> Option<usize> {
    if input.starts_with(const_str) {
        Some(const_str.len()) // Return the number of characters to skip
    } else {
        None
    }
}

fn skip_whitespace(chars: &mut CharIndices) {
    let mut whitespace_count = 0;
    let chars_clone = chars.clone();
    for (_, char) in chars_clone {
        if char.is_ascii_whitespace() {
            whitespace_count += 1;
        } else {
            break;
        }
    }
    if whitespace_count > 0 {
        chars.nth(whitespace_count - 1); // -1 because nth is 0-indexed
    }
}

/// test for parse_to_tokens
mod tests_parse_to_tokens {
    use super::*;

    #[test]
    fn test_parse_to_tokens() {
        let json = r#"
        {
            "name": "Jack (\"Bee\") Nimble", 
            "format": {
                "type":       "rect", 
                "width":      1920, 
                "height":     1080, 
                "interlace":  false,
                "array": [1,2,3,4,5,6,7,8,9,10]
            }
        }"#;
        let tokens = parse_to_tokens(json).unwrap();
        assert_eq!(
            tokens.tokens,
            vec![
                LeftBrace,
                String("name"),
                Colon,
                String("Jack (\\\"Bee\\\") Nimble"),
                Comma,
                String("format"),
                Colon,
                LeftBrace,
                String("type"),
                Colon,
                String("rect"),
                Comma,
                String("width"),
                Colon,
                Number(1920.0),
                Comma,
                String("height"),
                Colon,
                Number(1080.0),
                Comma,
                String("interlace"),
                Colon,
                False,
                Comma,
                String("array"),
                Colon,
                LeftBracket,
                Number(1.0),
                Comma,
                Number(2.0),
                Comma,
                Number(3.0),
                Comma,
                Number(4.0),
                Comma,
                Number(5.0),
                Comma,
                Number(6.0),
                Comma,
                Number(7.0),
                Comma,
                Number(8.0),
                Comma,
                Number(9.0),
                Comma,
                Number(10.0),
                RightBracket,
                RightBrace,
                RightBrace,
            ]
        );
    }
}
