#[cfg(test)]
mod test {
    use rust_json::parser::Parser;
    use rust_json::token::parse_to_tokens;
    use std::fs;
    use std::time::Instant;

    #[test]
    fn test() {
        use std::fs;
        use std::time::Instant;

        let mut token_times = vec![];
        let mut parse_times = vec![];
        let time = 10000;
        for i in 0..time {
            let json_text = fs::read_to_string(format!("tests/json{}.json", 1));
            if let Ok(text) = json_text {
                let start = Instant::now();
                let tokens = parse_to_tokens(&text).unwrap();
                let token_time = start.elapsed();
                token_times.push(token_time);

                let start = Instant::now();
                let _object = Parser::new(&tokens).parse();
                let parse_time = start.elapsed();
                parse_times.push(parse_time);
            } else {
                break;
            }
        }
        let total_token_time: u128 = token_times.iter().map(|d| d.as_nanos()).sum();
        let avg_token_time = total_token_time as f64 / token_times.len() as f64;

        let total_parse_time: u128 = parse_times.iter().map(|d| d.as_nanos()).sum();
        let avg_parse_time = total_parse_time as f64 / parse_times.len() as f64;

        println!("Average token time: {} ns", avg_token_time);
        println!("Average parse time: {} ns", avg_parse_time);
        println!("token + parse time:{} ns", avg_parse_time + avg_token_time);
    }
}
