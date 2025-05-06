use std::result::Result;

use crate::crux::error::ParseError;
use crate::crux::token::{ Token, TokenType, Object };
use super::expr;

pub struct Parser {

    tokens: Vec<Token>,
    current: usize,
    had_error: bool

}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, had_error: false }
    }

    pub fn parse(&mut self) -> Result<expr::Expr, ParseError> {

        let expr = self.expression()?;

        if self.had_error {
            Err(ParseError::SyntaxError {
                token: self.peek().clone(),
                message: "Parser encountered errors".into(),
            })
        }
        else {
            Ok(expr)
        }

    }

    fn expression(&mut self) -> Result<expr::Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<expr::Expr, ParseError>  {

        let mut expr = self.comparison()?;

        while self.rmatch(&[TokenType::BangEqual, TokenType::EqualEqual])? {
            let operator = self.previous().clone();
            let right = Box::new(self.comparison()?);
            expr = expr::Expr::Binary {
                left: Box::new(expr),
                operator,
                right
            };
        }

        Ok(expr)

    }

    fn comparison(&mut self) -> Result<expr::Expr, ParseError>  {

        let mut expr = self.term()?;

        while self.rmatch(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual])? {
            let operator = self.previous().clone();
            let right = Box::new(self.term()?);
            expr = expr::Expr::Binary {
                left: Box::new(expr),
                operator,
                right
            };
        }

        Ok(expr)

    }

    fn term(&mut self) -> Result<expr::Expr, ParseError> {

        let mut expr = self.factor()?;

        while self.rmatch(&[TokenType::Minus, TokenType::Plus])? {
            let operator = self.previous().clone();
            let right = Box::new(self.factor()?);
            expr = expr::Expr::Binary {
                left: Box::new(expr),
                operator,
                right
            }
        }

        Ok(expr)

    }

    fn factor(&mut self) -> Result<expr::Expr, ParseError> {

        let mut expr = self.unary()?;

        while self.rmatch(&[TokenType::Slash, TokenType::Star])? {
            let operator = self.previous().clone();
            let right = Box::new(self.unary()?);
            expr = expr::Expr::Binary {
                left: Box::new(expr),
                operator,
                right
            }
        }

        Ok(expr)

    }

    fn unary(&mut self) -> Result<expr::Expr, ParseError> {

        if self.rmatch(&[TokenType::Bang, TokenType::Minus])? {
            let operator= self.previous().clone();
            let right = Box::new(self.unary()?);
            Ok(expr::Expr::Unary {
                operator,
                right
            })
        }
        else {
            self.primary()
        }

    }

    fn primary(&mut self) -> Result<expr::Expr, ParseError> {

        if self.rmatch(&[TokenType::False])? {
            return Ok(expr::Expr::Literal {
                value: Object::Bool(false),
            });
        }

        if self.rmatch(&[TokenType::True])? {
            return Ok(expr::Expr::Literal {
                value: Object::Bool(true),
            });
        }

        if self.rmatch(&[TokenType::Non])? {
            return Ok(expr::Expr::Literal {
                value: Object::Non,
            });
        }

        if self.rmatch(&[TokenType::Number, TokenType::String])? {
            return Ok(expr::Expr::Literal {
                value: self.previous().literal.clone(),
            });
        }

        if self.rmatch(&[TokenType::LeftParen])? {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ) after expression")?;
            return Ok(expr::Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(ParseError::SyntaxError {
            token: self.peek().clone(),
            message: "Expected expression".into(),
        })

    }

    fn synchronize(&mut self) {

        self.advance();
        while !self.is_end() {

            if self.previous().token_type == TokenType::Semicolon { return }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fn
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {},
            }

            self.advance();

        }

    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, ParseError> {

        if self.check(token_type) {
            Ok(self.advance())
        }
        else {
            self.had_error = true;
            Err(ParseError::SyntaxError {
                token: self.peek().clone(),
                message: message.to_string(),
            })
        }

    }

    fn rmatch(&mut self, types: &[TokenType]) -> Result<bool, ParseError> {

        for ty in types {
            if self.check(ty) {
                self.advance();
                return Ok(true);
            }
        }

        Ok(false)

    }

    fn check(&self, token_type: &TokenType) -> bool {

        if self.is_end() {
            false
        }
        else {
            &self.peek().token_type == token_type
        }

    }

    fn advance(&mut self) -> &Token{

        if !self.is_end() {
            self.current += 1;
        }
        self.previous()

    }

    fn is_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

}
