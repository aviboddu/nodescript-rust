pub mod tokens;

pub fn tokenize_string(code: String) -> Result<tokens::Tokens, &'static str> {
    return tokens::tokenize(code);
}
