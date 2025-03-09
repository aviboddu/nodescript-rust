#[derive(PartialEq, Debug)]
pub struct Tokens {
    pub tokens: Vec<Vec<Token>>,
    pub code: String,
}

#[derive(PartialEq, Debug)]
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
    Identifier(String),
    String(String),
    Number(String),

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
        let tokens_for_line = match tokenize_line(line) {
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

fn tokenize_line<'a>(line: &str) -> Result<Vec<Token>, &'static str> {
    if line.is_empty() {
        return Ok(vec![]);
    }
    let bytes: &[u8] = line.as_bytes();
    let mut tokens: Vec<Token> = vec![];
    let mut start: usize = 0;
    let mut current: usize = 0;

    while current < bytes.len() {
        let c: char = bytes[current] as char;
        if c.is_whitespace() {
            current = current + 1;
            start = current;
            continue;
        }
        if c.eq(&'/') && (peek_next(bytes, current).eq(&'/')) {
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
                if peek_next(bytes, current).eq(&'=') {
                    tokens.push(Token::BangEqual);
                    current = current + 1
                } else {
                    tokens.push(Token::Bang);
                }
            }
            '=' => {
                if peek_next(bytes, current).eq(&'=') {
                    tokens.push(Token::EqualEqual);
                    current = current + 1;
                } else {
                    return Err("Unexpected Token");
                }
            }
            '<' => {
                if peek_next(bytes, current).eq(&'=') {
                    tokens.push(Token::LessEqual);
                    current = current + 1;
                } else {
                    tokens.push(Token::Less);
                }
            }
            '>' => {
                if peek_next(bytes, current).eq(&'=') {
                    tokens.push(Token::GreaterEqual);
                    current = current + 1;
                } else {
                    tokens.push(Token::Greater);
                }
            }
            '0'..='9' => {
                while peek_next(bytes, current).is_ascii_digit() {
                    current = current + 1;
                }
                tokens.push(Token::Number(line[start..=current].into()));
            }
            '"' => {
                while !peek_next(bytes, current).eq(&'"') {
                    if peek_next(bytes, current).eq(&'\0') {
                        return Err("Unmatched String");
                    }
                    current = current + 1;
                }
                tokens.push(Token::String(line[start..=current].into()));
            }
            'a'..='z' | 'A'..='Z' => {
                while peek_next(bytes, current).is_ascii_alphanumeric() {
                    current = current + 1;
                }
                tokens.push(get_keyword(line[start..=current].into()));
            }
            _ => return Err("Unexpected Token"),
        }
        current = current + 1;
        start = current;
    }
    return Ok(tokens);
}

fn peek_next(line: &[u8], current: usize) -> char {
    if current + 1 >= line.len() {
        return '\0';
    }
    return line[current + 1] as char;
}

fn get_keyword(identifier: &str) -> Token {
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
        _ => Token::Identifier(identifier.into()),
    }
}
