use crate::error::JsonParserError::UnexpectedToken;
use crate::token::JsonToken;

#[derive(Debug, PartialEq)]
pub enum JsonParserError<'a> {
    UnexpectedToken(UnexpectedTokenErrorDecr<'a>),
    InternalJsonParserError(JsonParserInternalError),
    UnexpectedEndOfTokens,
}
#[derive(Debug, PartialEq)]
pub struct UnexpectedTokenErrorDecr<'a> {
    pub expect: &'static str,
    pub actual: &'a JsonToken<'a>,
    pub msg: &'static str,
}

#[derive(Debug, PartialEq)]
pub enum JsonParserInternalError {
    TokenIndexOutOfRange,
}

pub fn expect_first_token_is_left_bracket_or_brace<'a>(
    token: &'a JsonToken,
) -> JsonParserError<'a> {
    UnexpectedToken(UnexpectedTokenErrorDecr {
        expect: "{ or [",
        actual: token,
        msg: "it should be { or [ on the first token for json value",
    })
}

pub fn expect_first_token_is_left_brace<'a>(token: &'a JsonToken) -> JsonParserError<'a> {
    UnexpectedToken(UnexpectedTokenErrorDecr {
        expect: "{",
        actual: token,
        msg: "it should be { on the first token for json object",
    })
}

pub fn expect_colon_after_key<'a>(token: &'a JsonToken) -> JsonParserError<'a> {
    UnexpectedToken(UnexpectedTokenErrorDecr {
        expect: ":",
        actual: token,
        msg: "it should be : after key",
    })
}

pub fn expect_a_comma_or_right_brace_after_value<'a>(token: &'a JsonToken) -> JsonParserError<'a> {
    UnexpectedToken(UnexpectedTokenErrorDecr {
        expect: ", or }",
        actual: token,
        msg: "it should be , or } after value",
    })
}

pub fn expect_key_or_right_brace<'a>(token: &'a JsonToken) -> JsonParserError<'a> {
    UnexpectedToken(UnexpectedTokenErrorDecr {
        expect: "} or key",
        actual: token,
        msg: "it should be value after :",
    })
}
