# rust_json

### A simple JSON data parser.

### Parsing Process

* parse input json text to JsonToken
```rust
pub enum JsonToken<'a> {
    LeftBrace,          // {
    RightBrace,         // }
    DoubleQuote,        // "
    Comma,              // ,
    Number(i64),        // 1,2,3,...
    Colon,              // :
    True,               // true
    False,              // false
    Null,               // null
    String(&'a str),    // "..."
    LeftBracket,        // [
    RightBracket,       // ]
}
```
* Converting structured JsonTokens into JsonObject.
```rust
pub struct JsonObject<'a> {
    children: Vec<KeyValue<'a>>,
}
```
KeyValue represents a key-value pair
```rust
#[derive(Debug)]
pub struct KeyValue<'a> {
    key: &'a str,
    value: JsonValue<'a>,
}
```
JsonValue is an enum represents all the json data type
```rust
pub enum JsonValue<'a> {
    Number(Box<i64>),
    String(Box<&'a str>),
    Object(Box<JsonObject<'a>>),
    Array(Box<JsonArray<'a>>),
    True,
    False,
    Null,
}
```
JsonArray represent json array
```rust
pub struct JsonArray<'a> {
    array: Vec<JsonValue<'a>>,
}
```

The default Display trait will represent the output of a JSON string without any spaces or newline characters.

For all parsed string objects, they will be interpreted as string references (with JsonToken acting as an intermediary layer), and any numerical objects will be copied.


