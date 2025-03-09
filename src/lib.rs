use std::{fs::read_to_string, path::Path};

mod tokens;

pub fn tokenize_file(path: &Path) -> Result<tokens::Tokens, &'static str> {
    let contents: Result<String, std::io::Error> = read_to_string(&path);
    let code: String = match contents {
        Ok(c) => c,
        Err(_c) => return Err("IO Error"),
    };
    return tokens::tokenize(code);
}

#[cfg(test)]
mod tests {
    use super::*;

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
