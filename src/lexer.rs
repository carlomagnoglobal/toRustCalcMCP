//! Lexer: tokenize source strings.

use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    // Literals
    Number(String),
    String(String),
    Ident(String),
    // Keywords
    Define,
    If,
    For,
    While,
    Print,
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    SlashSlash,  // //
    Percent,
    Caret,        // ^ (power)
    // Comparisons
    EqEq,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Equal,
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tok::Number(s) => write!(f, "Num({})", s),
            Tok::String(s) => write!(f, "Str({})", s),
            Tok::Ident(s) => write!(f, "Id({})", s),
            Tok::Define => write!(f, "define"),
            Tok::If => write!(f, "if"),
            Tok::For => write!(f, "for"),
            Tok::While => write!(f, "while"),
            Tok::Print => write!(f, "print"),
            Tok::Plus => write!(f, "+"),
            Tok::Minus => write!(f, "-"),
            Tok::Star => write!(f, "*"),
            Tok::Slash => write!(f, "/"),
            Tok::SlashSlash => write!(f, "//"),
            Tok::Percent => write!(f, "%"),
            Tok::Caret => write!(f, "^"),
            Tok::EqEq => write!(f, "=="),
            Tok::BangEq => write!(f, "!="),
            Tok::Lt => write!(f, "<"),
            Tok::LtEq => write!(f, "<="),
            Tok::Gt => write!(f, ">"),
            Tok::GtEq => write!(f, ">="),
            Tok::LParen => write!(f, "("),
            Tok::RParen => write!(f, ")"),
            Tok::LBrace => write!(f, "{{"),
            Tok::RBrace => write!(f, "}}"),
            Tok::Comma => write!(f, ","),
            Tok::Semicolon => write!(f, ";"),
            Tok::Equal => write!(f, "="),
        }
    }
}

pub fn lex(src: &str) -> Result<Vec<Tok>, String> {
    let mut toks = Vec::new();
    let mut chars = src.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Comments
            '#' => {
                chars.next();
                while chars.peek().is_some() && chars.peek() != Some(&'\n') {
                    chars.next();
                }
            }
            // Operators and delimiters
            '+' => {
                chars.next();
                toks.push(Tok::Plus);
            }
            '-' => {
                chars.next();
                toks.push(Tok::Minus);
            }
            '*' => {
                chars.next();
                if chars.peek() == Some(&'*') {
                    chars.next();
                    toks.push(Tok::Caret); // ** -> ^
                } else {
                    toks.push(Tok::Star);
                }
            }
            '/' => {
                chars.next();
                if chars.peek() == Some(&'/') {
                    chars.next();
                    toks.push(Tok::SlashSlash);
                } else {
                    toks.push(Tok::Slash);
                }
            }
            '%' => {
                chars.next();
                toks.push(Tok::Percent);
            }
            '^' => {
                chars.next();
                toks.push(Tok::Caret);
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    toks.push(Tok::EqEq);
                } else {
                    toks.push(Tok::Equal);
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    toks.push(Tok::BangEq);
                } else {
                    return Err("unexpected '!'".to_string());
                }
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    toks.push(Tok::LtEq);
                } else {
                    toks.push(Tok::Lt);
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    toks.push(Tok::GtEq);
                } else {
                    toks.push(Tok::Gt);
                }
            }
            '(' => {
                chars.next();
                toks.push(Tok::LParen);
            }
            ')' => {
                chars.next();
                toks.push(Tok::RParen);
            }
            '{' => {
                chars.next();
                toks.push(Tok::LBrace);
            }
            '}' => {
                chars.next();
                toks.push(Tok::RBrace);
            }
            ',' => {
                chars.next();
                toks.push(Tok::Comma);
            }
            ';' => {
                chars.next();
                toks.push(Tok::Semicolon);
            }
            '"' => {
                chars.next();
                let mut s = String::new();
                while chars.peek().is_some() && chars.peek() != Some(&'"') {
                    if let Some(c) = chars.next() {
                        s.push(c);
                    }
                }
                if chars.peek() == Some(&'"') {
                    chars.next();
                } else {
                    return Err("unterminated string".to_string());
                }
                toks.push(Tok::String(s));
            }
            _ if ch.is_ascii_digit() || (ch == '.' && chars.clone().nth(1).map_or(false, |c| c.is_ascii_digit())) => {
                toks.push(lex_number(&mut chars)?);
            }
            _ if ch.is_ascii_alphabetic() || ch == '_' => {
                let tok = lex_ident(&mut chars);
                toks.push(tok);
            }
            _ => {
                return Err(format!("unexpected character: {}", ch));
            }
        }
    }

    Ok(toks)
}

fn lex_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Result<Tok, String> {
    let mut num = String::new();

    // handle hex/binary prefixes
    if chars.peek() == Some(&'0') {
        num.push(chars.next().unwrap());
        if chars.peek() == Some(&'x') || chars.peek() == Some(&'X') {
            num.push(chars.next().unwrap());
            while chars.peek().map_or(false, |c| c.is_ascii_hexdigit()) {
                num.push(chars.next().unwrap());
            }
            return Ok(Tok::Number(num));
        } else if chars.peek() == Some(&'b') || chars.peek() == Some(&'B') {
            num.push(chars.next().unwrap());
            while chars.peek() == Some(&'0') || chars.peek() == Some(&'1') {
                num.push(chars.next().unwrap());
            }
            return Ok(Tok::Number(num));
        }
    }

    // decimal integer part
    while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
        num.push(chars.next().unwrap());
    }

    // fractional part
    if chars.peek() == Some(&'.') {
        num.push(chars.next().unwrap());
        while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
            num.push(chars.next().unwrap());
        }
    }

    // exponent
    if chars.peek() == Some(&'e') || chars.peek() == Some(&'E') {
        num.push(chars.next().unwrap());
        if chars.peek() == Some(&'+') || chars.peek() == Some(&'-') {
            num.push(chars.next().unwrap());
        }
        while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
            num.push(chars.next().unwrap());
        }
    }

    Ok(Tok::Number(num))
}

fn lex_ident(chars: &mut std::iter::Peekable<std::str::Chars>) -> Tok {
    let mut ident = String::new();
    while chars.peek().map_or(false, |c| c.is_ascii_alphanumeric() || *c == '_') {
        ident.push(chars.next().unwrap());
    }
    match ident.as_str() {
        "define" => Tok::Define,
        "if" => Tok::If,
        "for" => Tok::For,
        "while" => Tok::While,
        "print" => Tok::Print,
        _ => Tok::Ident(ident),
    }
}
