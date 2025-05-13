use std::result::Result;

use crate::crux::error::ParseError;
use crate::crux::token::{ Token, TokenType, Object };
use super::expr;
use crate::backend::stmt::Stmt;

pub struct Parser {

    tokens: Vec<Token>,
    current: usize,
    had_error: bool

}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, had_error: false }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {

        let mut statements = Vec::new();
        while !self.is_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(statements)

    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {

        if self.rmatch(&[TokenType::Print])? {
            Ok(self.print_statement()?)
        }
        else if self.rmatch(&[TokenType::PrintLn])? {
            Ok(self.println_statement()?)
        }
        else if self.rmatch(&[TokenType::LeftBrace])? {
            Ok(self.block()?)
        }
        else if self.rmatch(&[TokenType::If])? {
            Ok(self.if_statement()?)
        }
        else {
            Ok(self.expression_statement()?)
        }

    }

    fn if_statement(&self) -> Result<Stmt, ParseError> {

        self.consume(&TokenType::LeftParen, "Expected ')' after 'if'")?;
        let condition = self.expression()?;
        self.consume(&TokenType::RightParen, "Expected ')' after if condition")?;

        let then_branch = self.statement()?;

    }

    fn block(&mut self) -> Result<Stmt, ParseError> {

        let mut statements: Vec<Stmt> = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expected a } after block")?;
        Ok(Stmt::Block {
            statements
        })

    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {

        match self.rmatch(&[TokenType::Let]) {
            Ok(true) => Ok(self.var_declaration()?),
            Ok(false) => Ok(self.statement()?),
            Err(_) => {
                self.synchronize();
                Err(ParseError::SyntaxError {
                    token: self.peek().clone(),
                    message: "Expected expression".into(),
                })
            }
        }

    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {

        let value = self.expression()?;
        match self.consume(&TokenType::Semicolon, "Expected ; after value") {
            Ok(_) => Ok(Stmt::Print { expression: Box::new(value) }),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }

    }

    fn println_statement(&mut self) -> Result<Stmt, ParseError> {

        let value = self.expression()?;
        match self.consume(&TokenType::Semicolon, "Expected ; after value") {
            Ok(_) => Ok(Stmt::PrintLn { expression: Box::new(value) }),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }

    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {

        let name = self.consume(&TokenType::Identifier, "Expect variable name")?.clone();

        let initializer = if self.rmatch(&[TokenType::Equal])? {
            Some(self.expression()?)
        }
        else {
            None
        };

        self.consume(&TokenType::Semicolon, "Expect ';' after variable declaration")?;

        Ok(Stmt::Let {
            name,
            initializer: Box::new(initializer.unwrap_or(expr::Expr::Literal {
                value: Object::Null,
            })),
        })

    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {

        let expr = self.expression()?;
        match self.consume(&TokenType::Semicolon, "Expected ; after expression") {
            Ok(_) => Ok(Stmt::Expression { expression: Box::new(expr) }),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }

    }

    fn expression(&mut self) -> Result<expr::Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<expr::Expr, ParseError> {

        let expr = self.equality()?;

        if self.rmatch(&[TokenType::Equal])? {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                expr::Expr::Variable { name } => {
                    Ok(expr::Expr::Assign {
                        name,
                        value: Box::new(value),
                    })
                }
                _ => { Err(ParseError::SyntaxError {
                    token: equals.clone(),
                    message: "Invalid assignment target ".into(), })
                }
            }

        }
        else {
            Ok(expr)
        }

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

        if self.rmatch(&[TokenType::Null])? {
            return Ok(expr::Expr::Literal {
                value: Object::Null,
            });
        }

        if self.rmatch(&[TokenType::Number, TokenType::String])? {
            return Ok(expr::Expr::Literal {
                value: self.previous().literal.clone(),
            });
        }

        if self.rmatch(&[TokenType::Identifier])? {
            return Ok(expr::Expr::Variable {
                name: self.previous().clone()
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
                | TokenType::PrintLn
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
            Err(ParseError::SyntaxError {
                token: self.previous().clone(),
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
