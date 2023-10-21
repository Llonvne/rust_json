use std::str::CharIndices;
use JsonToken::*;

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

/// 将给定的 origin 字符串解析为 JsonToken
/// 所有字符串都使用 &'a str 与 origin 字符串引用统一生命周期
/// i8,i16,..,u8,u16,..,f32,f64 均使用赋值
///
/// # Arguments
///
/// * `origin`:
///
/// returns: Option<Vec<JsonToken, Global>>
///
/// # Examples
///
/// ```
///
/// ```
pub fn parse_to_tokens(origin: &str) -> Option<JsonTokenStream> {
    let mut tokens: Vec<JsonToken> = vec![];
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
                if let Some((number, _)) = parse_number(&mut char_indices, index, origin) {
                    tokens.push(Number(number));
                }
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
    Some(JsonTokenStream { tokens })
}

fn parse_number(
    char_indices: &mut CharIndices<'_>,
    current_index: usize,
    origin: &str,
) -> Option<(f64, usize)> {
    let mut peek = char_indices.clone().peekable();
    let start = current_index;
    let mut end = current_index;
    let mut has_dot = false;

    for (_index, char) in peek {
        if char == '.' {
            if has_dot {
                panic!("already has dot")
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
        Ok(number) => Some((number, end)),
        Err(_) => panic!("parse number error"),
    }
}

fn parse_key(input: &str) -> Option<(&str, usize)> {
    let mut char_indices = input.char_indices();
    let mut start_index = 0;
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

    match (end_index) {
        Some(end) => Some((&input[start_index..end], end)),
        _ => panic!("parse key error"),
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
