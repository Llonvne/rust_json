#[cfg(test)]
mod test {
    use rust_json::parser::Parser;
    use rust_json::token::parse_to_tokens;
    use std::fs;

    #[test]
    fn test() {
        let json_text = fs::read_to_string("tests/json1.json").unwrap();
        let tokens = parse_to_tokens(&json_text).unwrap();
        let object = Parser::new(&tokens).parse();
        println!("{}", object)
    }
}
