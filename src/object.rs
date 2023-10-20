use crate::r#type::JsonType;

#[derive(Debug)]
pub struct JsonObject<'a> {
    pub origin: &'a str,
    pub values: Vec<JsonKeyPair<'a>>,
}

#[derive(Debug)]
pub struct JsonKeyPair<'a> {
    pub key: &'a str,
    pub value: JsonType<'a>,
}
