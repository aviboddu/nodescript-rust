use std::{fs::read_to_string, path::Path};

mod tests;
mod tokens;

pub fn tokenize_file(path: &Path) -> Result<tokens::Tokens, &'static str> {
    let contents: Result<String, std::io::Error> = read_to_string(&path);
    let code: String = match contents {
        Ok(c) => c,
        Err(_c) => return Err("IO Error"),
    };
    return tokens::tokenize(code);
}
