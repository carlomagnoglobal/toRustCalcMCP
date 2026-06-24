//! Parser: build an AST from tokens using Pratt parsing.

use crate::lexer::{lex, Tok};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number(String),
    Str(String),
    Var(String),
    Assign(String, Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Pos,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

struct Parser {
    toks: Vec<Tok>,
    pos: usize,
}

impl Parser {
    fn new(toks: Vec<Tok>) -> Self {
        Parser { toks, pos: 0 }
    }

    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn advance(&mut self) -> Option<Tok> {
        if self.pos < self.toks.len() {
            let tok = self.toks[self.pos].clone();
            self.pos += 1;
            Some(tok)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: Tok) -> Result<(), String> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("expected {:?}, found {:?}", expected, self.peek()))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let expr = self.parse_logical_or()?;
        if let Expr::Var(name) = &expr {
            if self.peek() == Some(&Tok::Equal) {
                self.advance();
                let rhs = self.parse_assignment()?;
                return Ok(Expr::Assign(name.clone(), Box::new(rhs)));
            }
        }
        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_addition()?;

        loop {
            let op = match self.peek() {
                Some(Tok::EqEq) => BinOp::Eq,
                Some(Tok::BangEq) => BinOp::Ne,
                Some(Tok::Lt) => BinOp::Lt,
                Some(Tok::LtEq) => BinOp::Le,
                Some(Tok::Gt) => BinOp::Gt,
                Some(Tok::GtEq) => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_addition()?;
            expr = Expr::Binary(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_multiplication()?;

        loop {
            let op = match self.peek() {
                Some(Tok::Plus) => BinOp::Add,
                Some(Tok::Minus) => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication()?;
            expr = Expr::Binary(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_power()?;

        loop {
            let op = match self.peek() {
                Some(Tok::Star) => BinOp::Mul,
                Some(Tok::Slash) => BinOp::Div,
                Some(Tok::SlashSlash) => BinOp::IntDiv,
                Some(Tok::Percent) => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            expr = Expr::Binary(op, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_unary()?;

        // ^ is right-associative
        if self.peek() == Some(&Tok::Caret) {
            self.advance();
            let right = self.parse_power()?;
            expr = Expr::Binary(BinOp::Pow, Box::new(expr), Box::new(right));
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Tok::Minus) => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnOp::Neg, Box::new(expr)))
            }
            Some(Tok::Plus) => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::Unary(UnOp::Pos, Box::new(expr)))
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.peek() == Some(&Tok::LParen) {
                // Function call
                if let Expr::Var(name) = expr {
                    self.advance();
                    let args = self.parse_args()?;
                    self.expect(Tok::RParen)?;
                    expr = Expr::Call(name, args);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Tok::Number(s)) => Ok(Expr::Number(s)),
            Some(Tok::String(s)) => Ok(Expr::Str(s)),
            Some(Tok::Ident(s)) => Ok(Expr::Var(s)),
            Some(Tok::LParen) => {
                let expr = self.parse_expr()?;
                self.expect(Tok::RParen)?;
                Ok(expr)
            }
            _ => Err("expected expression".to_string()),
        }
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        if self.peek() != Some(&Tok::RParen) {
            loop {
                args.push(self.parse_expr()?);
                if self.peek() == Some(&Tok::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        Ok(args)
    }
}

pub fn parse(src: &str) -> Result<Vec<Expr>, String> {
    let toks = lex(src)?;
    let mut parser = Parser::new(toks);
    let mut exprs = Vec::new();

    while parser.peek().is_some() {
        if parser.peek() == Some(&Tok::Semicolon) {
            parser.advance();
            continue;
        }
        exprs.push(parser.parse_expr()?);
        if parser.peek() == Some(&Tok::Semicolon) {
            parser.advance();
        } else if parser.peek().is_some() {
            return Err("expected ; or end of input".to_string());
        }
    }

    Ok(exprs)
}
