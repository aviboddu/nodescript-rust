use std::fmt;

use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub tokens: Vec<Vec<Token>>,
    pub code: String,
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.tokens {
            for token in line {
                write!(f, "{} ", token)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
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
    /*EQUAL,*/
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftSquare => write!(f, "["),
            Token::RightSquare => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Slash => write!(f, "/"),
            Token::Star => write!(f, "*"),
            Token::Colon => write!(f, ":"),
            Token::Bang => write!(f, "!"),
            Token::BangEqual => write!(f, "!="),
            Token::EqualEqual => write!(f, "=="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::Identifier(start, end) => {
                write!(f, "Identifier({}, {})", start, end)
            }
            Token::String(start, end) => {
                write!(f, "String({}, {})", start, end)
            }
            Token::Number(start, end) => {
                write!(f, "Number({}, {})", start, end)
            }
            Token::And => write!(f, "and"),
            Token::Else => write!(f, "ELSE"),
            Token::False => write!(f, "false"),
            Token::If => write!(f, "IF"),
            Token::Or => write!(f, "or"),
            Token::EndIf => write!(f, "ENDIF"),
            Token::Set => write!(f, "SET"),
            Token::Nop => write!(f, "NOP"),
            Token::Print => write!(f, "PRINT"),
            Token::Return => write!(f, "RETURN"),
            Token::True => write!(f, "true"),
            Token::Eof => write!(f, "eof"),
        }
    }
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

fn tokenize_line<'a>(code: &str, offset: usize, len: usize) -> Result<Vec<Token>, &'static str> {
    if len == 0 {
        return Ok(vec![]);
    }
    let bytes: &[u8] = code.as_bytes();
    let end: usize = offset + len;

    let mut tokens: Vec<Token> = vec![];
    let mut start: usize = offset;
    let mut current: usize = offset;

    while current < len {
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
                tokens.push(Token::String(start, current));
            }
            'a'..='z' | 'A'..='Z' => {
                while peek_next(bytes, current, end).is_ascii_alphanumeric() {
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
