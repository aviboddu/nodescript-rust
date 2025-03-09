#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn basic_parse() {
        let path = Path::new("./tests/Minimal.ns");
        let result = tokenize_file(path).unwrap();
        assert_eq!(
            result,
            tokens::Tokens {
                tokens: vec![vec![
                    tokens::Token::Print,
                    tokens::Token::Number("0".into()),
                    tokens::Token::Comma,
                    tokens::Token::Identifier("input".into()),
                    tokens::Token::Eof,
                ],],
                code: "PRINT 0, input".into(),
            }
        );
    }
}
