use regex::Regex;

lazy_static::lazy_static! {
    static ref ADDR_TOKENS: Regex = Regex::new(r"\(*\b[^\s,;#&()]+[.,;)\n]*|[#&]").unwrap();
}

lazy_static::lazy_static! {
    static ref AMPERSAND: Regex = Regex::new("(&#38;)|(&amp;)").unwrap();
}

pub fn tokenize(address: &str) -> Vec<String> {
    let address = AMPERSAND.replace_all(address, "&");
    ADDR_TOKENS
        .find_iter(&address)
        .map(|c| c.as_str().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(
            tokenize("# 1 abc st"),
            vec![
                "#".to_string(),
                "1".to_string(),
                "abc".to_string(),
                "st".to_string()
            ]
        );

        assert_eq!(
            tokenize("#1 abc st"),
            vec![
                "#".to_string(),
                "1".to_string(),
                "abc".to_string(),
                "st".to_string()
            ]
        );
    }
}
