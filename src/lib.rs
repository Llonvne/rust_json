use crate::JsonParseError::*;
use crate::JsonToken::*;
use crate::ParserStatus::{
    ExceptColon, ExceptCommaOrRightBrace, ExceptValue, ExpectKey, ExpectKeyOrRightBrace,
};
use std::fmt::{Display, Formatter, Write};
use std::str::{CharIndices, Chars};

#[derive(Debug)]
pub enum JsonToken<'a> {
    LeftBrace,
    RightBrace,
    WhiteSpace,
    DoubleQuote,
    Identifier(&'a str),
    Comma,
    Number(i64),
    Colon,
    True,
    False,
    Null,
    String(&'a str),
    LeftBracket,
    RightBracket,
}
impl<'a> Display for JsonToken<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            WhiteSpace => write!(f, " "),
            DoubleQuote => write!(f, "\""),
            Identifier(id) => write!(f, "\"{}\"", id),
            Comma => write!(f, ","),
            Number(num) => write!(f, "{}", num),
            Colon => write!(f, ":"),
            True => write!(f, "true"),
            False => write!(f, "false"),
            Null => write!(f, "null"),
            String(str) => write!(f, "\"{}\"", *str),
            LeftBracket => write!(f, "["),
            RightBracket => write!(f, "]"),
        }
    }
}
impl<'a> Display for Tokens<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut indent_level = 0;
        for token in &self.0 {
            if let Identifier(_) = token {
                write_indent(f, indent_level);
            }
            if let RightBrace = token {
                write!(f, "\n")?;
                if indent_level > 0 {
                    indent_level -= 1;
                }
            }
            write!(f, "{}", token)?;
            if let LeftBrace = token {
                write!(f, "\n")?;
                indent_level += 1;
            }

            if let Comma = token {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
fn write_indent(f: &mut Formatter<'_>, indent_level: usize) -> std::fmt::Result {
    for _ in 0..indent_level {
        f.write_str("    ")?; // Four spaces per indent level
    }
    Ok(())
}
pub struct Parser<'a> {
    origin: &'a str,
}
enum ParserStatus {
    ExpectLeftBrace,
    ExpectKeyOrRightBrace,
    ExceptColon,
    ExceptValue,
    ExceptCommaOrRightBrace,
    ExpectKey,
}
#[derive(Debug)]
enum JsonParseError {
    ExpectLeftBrace,
    ExpectRightBrace,
    ExpectComma,
    NumberFormatErr,
    ExpectKey,
    KeyError,
    ValueError,
    ExpectLeftBracket,
}
impl<'a> Parser<'a> {
    pub(crate) fn parse(&mut self) -> Result<Tokens<'a>, JsonParseError> {
        let mut tokens = vec![];
        let mut state = ParserStatus::ExpectLeftBrace;
        let mut char_indices = self.origin.char_indices();
        let mut depth = 0;

        while let Some((index, char)) = char_indices.next() {
            // 省去所有不必要的空格
            skip_whitespace(&mut char_indices);
            match state {
                ParserStatus::ExpectLeftBrace => {
                    if char == '{' {
                        handle_left_brace(&mut tokens, &mut state, &mut depth);
                    } else {
                        return Err(ExpectLeftBrace);
                    }
                }

                ExceptValue => {
                    state = ExceptCommaOrRightBrace;
                    match char {
                        '{' => {
                            handle_left_brace(&mut tokens, &mut state, &mut depth);
                        }
                        '"' => {
                            let (str, end) = parse_key(char_indices.clone().as_str())?;
                            char_indices.nth(end);
                            tokens.push(String(str));
                        }
                        '0'..='9' => match parse_number(&mut char_indices, index, self.origin) {
                            Ok((number, end_index)) => {
                                tokens.push(JsonToken::Number(number));
                            }
                            Err(e) => return Err(e),
                        },
                        '[' => {
                            tokens.push(LeftBracket);
                        }
                        // 匹配 true false null
                        't' | 'f' | 'n' => {
                            if parse_const_if_ok_then_skip(
                                char_indices.clone().as_str(),
                                "rue",
                                &mut char_indices,
                            )
                            .is_ok()
                            {
                                tokens.push(True);
                            }
                            if parse_const_if_ok_then_skip(
                                char_indices.clone().as_str(),
                                "alse",
                                &mut char_indices,
                            )
                            .is_ok()
                            {
                                tokens.push(False);
                            }
                            if parse_const_if_ok_then_skip(
                                char_indices.clone().as_str(),
                                "ull",
                                &mut char_indices,
                            )
                            .is_ok()
                            {
                                tokens.push(JsonToken::Null);
                            }
                        }
                        _ => return Err(ValueError),
                    }
                }

                ExceptColon => {
                    if char.is_ascii_whitespace() {
                        continue;
                    } else if char == ':' {
                        tokens.push(JsonToken::Colon);
                        state = ExceptValue
                    } else {
                        return Err(ExpectComma);
                    }
                }

                ExpectKeyOrRightBrace => {
                    if char == '"' {
                        let (str, end) = parse_key(char_indices.clone().as_str())?;
                        char_indices.nth(end);
                        tokens.push(JsonToken::Identifier(str));
                        state = ExceptColon;
                    } else if char == '}' {
                        handle_right_brace(&mut tokens, &mut state, &mut depth)
                    }
                }
                ExceptCommaOrRightBrace => {
                    if char == ',' {
                        tokens.push(JsonToken::Comma);
                        state = ExpectKey;
                    } else if char == '}' {
                        handle_right_brace(&mut tokens, &mut state, &mut depth)
                    }
                }
                ExpectKey => {
                    if char == '"' {
                        let (str, end) = parse_key(char_indices.clone().as_str())?;
                        char_indices.nth(end);
                        tokens.push(JsonToken::Identifier(str));
                        state = ExceptColon;
                    } else {
                        return Err(JsonParseError::ExpectKey);
                    }
                }
            }
        }
        // if depth == 0 {
        //     Ok(Tokens(tokens))
        // } else {
        //     Err(ExpectRightBrace)
        // }
        Ok(Tokens(tokens))
    }
}

fn parse_number(
    char_indices: &mut std::str::CharIndices<'_>,
    current_index: usize,
    origin: &str,
) -> Result<(i64, usize), JsonParseError> {
    let mut peek = char_indices.clone().peekable();
    let start = current_index;
    let mut end = current_index;
    for (_index, char) in peek {
        if char.is_ascii_digit() {
            end = _index;
        } else {
            break;
        }
    }

    // Sync the state to char_indices
    if end - start > 1 {
        char_indices.nth(end - start - 1);
    }

    let number_str = &origin[start..end + 1];
    match number_str.parse::<i64>() {
        Ok(number) => Ok((number, end)),
        Err(_) => Err(NumberFormatErr),
    }
}

fn parse_key(input: &str) -> Result<(&str, usize), JsonParseError> {
    let mut char_indices = input.char_indices();
    let mut start_index = 0;
    let mut end_index = None;

    for (index, char) in char_indices {
        match char {
            '"' => {
                // This is the closing quote of the key
                end_index = Some(index);
                break;
            }
            _ => continue,
        }
    }

    match (end_index) {
        Some(end) => Ok((&input[start_index..end], end)),
        _ => Err(KeyError),
    }
}

fn handle_left_brace(tokens: &mut Vec<JsonToken<'_>>, state: &mut ParserStatus, depth: &mut usize) {
    tokens.push(LeftBrace);
    *state = ExpectKeyOrRightBrace;
    *depth += 1;
}

fn handle_right_brace(
    tokens: &mut Vec<JsonToken<'_>>,
    state: &mut ParserStatus,
    depth: &mut usize,
) {
    tokens.push(RightBrace);
    *state = ExceptCommaOrRightBrace;
    if *depth > 1 {
        *depth -= 1;
    }
}

fn parse_const_if_ok_then_skip(
    input: &str,
    pattern: &str,
    chars: &mut CharIndices,
) -> Result<(), JsonParseError> {
    match parse_const(input, pattern) {
        Ok(size) => {
            chars.nth(size - 1);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn parse_const(input: &str, const_str: &str) -> Result<usize, JsonParseError> {
    if input.starts_with(const_str) {
        Ok(const_str.len()) // Return the number of characters to skip
    } else {
        Err(ValueError) // Assume ValueError is a variant of JsonParseError
    }
}

fn skip_whitespace(chars: &mut CharIndices) {
    let mut whitespace_count = 0;
    for (_, char) in chars.clone() {
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

pub struct Tokens<'a>(Vec<JsonToken<'a>>);

pub fn parse(json_text: &str) {
    let mut parser = Parser { origin: json_text };

    let tokens = parser.parse();
    println!("{}", tokens.unwrap())
}
