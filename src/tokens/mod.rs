use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub tokens: Vec<Vec<Token>>,
    pub code: String,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Colon,
    // One or two character tokens.
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(usize, usize),
    String(usize, usize),
    Number(usize, usize),

    // Keywords.
    And,
    Else,
    False,
    If,
    Or,
    EndIf,
    Set,
    Nop,
    Print,
    Return,
    True,
    Eof,
}

pub fn tokenize(code: String) -> Result<Tokens, &'static str> {
    if code.is_empty() {
        return Ok(Tokens {
            tokens: vec![],
            code,
        });
    }
    let mut tokens: Vec<Vec<Token>> = vec![];
    for line in code.lines() {
        let offset: usize = unsafe { line.as_ptr().byte_offset_from(code.as_ptr()) as usize };
        let len: usize = line.len();
        let tokens_for_line = match tokenize_line(code.as_str(), offset, len) {
            Ok(t) => t,
            Err(r) => return Err(r),
        };
        tokens.push(tokens_for_line);
    }
    match tokens.last_mut() {
        Some(v) => v.push(Token::Eof),
        None => {}
    };
    return Ok(Tokens { tokens, code });
}

fn tokenize_line(code: &str, offset: usize, len: usize) -> Result<Vec<Token>, &'static str> {
    if len == 0 {
        return Ok(vec![]);
    }
    let bytes: &[u8] = code.as_bytes();
    let end: usize = offset + len;

    let mut tokens: Vec<Token> = vec![];
    let mut start: usize = offset;
    let mut current: usize = offset;

    while current < end {
        let c: char = bytes[current] as char;
        if c.is_whitespace() {
            current = current + 1;
            start = current;
            continue;
        }
        if c.eq(&'/') && (peek_next(bytes, current, end).eq(&'/')) {
            break;
        }
        match c {
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '[' => tokens.push(Token::LeftSquare),
            ']' => tokens.push(Token::RightSquare),
            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),
            '-' => tokens.push(Token::Minus),
            '+' => tokens.push(Token::Plus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            ':' => tokens.push(Token::Colon),
            '!' => {
                if peek_next(bytes, current, end).eq(&'=') {
                    tokens.push(Token::BangEqual);
                    current = current + 1
                } else {
                    tokens.push(Token::Bang);
                }
            }
            '=' => {
                if peek_next(bytes, current, end).eq(&'=') {
                    tokens.push(Token::EqualEqual);
                    current = current + 1;
                } else {
                    return Err("Unexpected Token");
                }
            }
            '<' => {
                if peek_next(bytes, current, end).eq(&'=') {
                    tokens.push(Token::LessEqual);
                    current = current + 1;
                } else {
                    tokens.push(Token::Less);
                }
            }
            '>' => {
                if peek_next(bytes, current, end).eq(&'=') {
                    tokens.push(Token::GreaterEqual);
                    current = current + 1;
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '0'..='9' => {
                while peek_next(bytes, current, end).is_ascii_digit() {
                    current = current + 1;
                }
                tokens.push(Token::Number(start, current));
            }
            '"' => {
                while !peek_next(bytes, current, end).eq(&'"') {
                    if peek_next(bytes, current, end).eq(&'\0') {
                        return Err("Unmatched String");
                    }
                    current = current + 1;
                }
                current = current + 1;
                tokens.push(Token::String(start, current));
            }
            'a'..='z' | 'A'..='Z' => {
                while peek_next(bytes, current, end).is_ascii_alphanumeric()
                    || peek_next(bytes, current, end).eq(&'_')
                {
                    current = current + 1;
                }
                tokens.push(get_keyword(bytes, start, current));
            }
            _ => return Err("Unexpected Token"),
        }
        current = current + 1;
        start = current;
    }
    return Ok(tokens);
}

fn peek_next(line: &[u8], current: usize, end: usize) -> char {
    if current + 1 >= end {
        return '\0';
    }
    return line[current + 1] as char;
}

fn get_keyword(code: &[u8], start: usize, current: usize) -> Token {
    let identifier: &str = unsafe { std::str::from_utf8_unchecked(&code[start..=current]) };
    match identifier {
        "and" => Token::And,
        "ELSE" => Token::Else,
        "false" => Token::False,
        "IF" => Token::If,
        "or" => Token::Or,
        "ENDIF" => Token::EndIf,
        "SET" => Token::Set,
        "NOP" => Token::Nop,
        "PRINT" => Token::Print,
        "RETURN" => Token::Return,
        "true" => Token::True,
        _ => Token::Identifier(start, current),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{File, read_dir, read_to_string},
        io::BufReader,
        path::Path,
    };

    use crate::tokens::*;

    #[test]
    fn basic_parse() {
        let data = read_dir(Path::new("./data/")).expect("Failed to read data directory");

        for base_path in data {
            let base_path = match base_path {
                Ok(folder) if folder.file_type().unwrap().is_dir() => folder.path(),
                _ => continue,
            };
            let script = base_path.join("script.ns");
            let result = match tokenize(read_to_string(&script).expect("Failed to read file")) {
                Ok(tokens) => tokens,
                Err(e) => panic!("Failed to tokenize: {}", e),
            };

            let cmp_path = base_path.join("tokens.tok");
            let file = File::open(&cmp_path).expect("Failed to open token file");
            let reader = BufReader::new(file);
            let cmp: Tokens =
                serde_json::from_reader(reader).expect("Failed to deserialize token file");
            println!("Successfully tokenized file {}", script.display());
            assert_eq!(result, cmp)
        }
    }
}
