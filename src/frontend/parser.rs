use super::token::{ Token, TokenType, Object };
use super::expr;

pub struct Parser {

    tokens: Vec<Token>,
    current: usize

}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> expr::Expr {
        self.equality()
    }

    fn equality(&mut self) -> expr::Expr {

        let mut expr = self.comparison();

        while self.rmatch(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparison());
            expr = expr::Expr::Binary {
                left: Box::new(expr),
                operator,
                right
            };
        }

        expr

    }

    fn comparison(&mut self) -> expr::Expr {

        let mut expr = self.term();

        while self.rmatch(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone();
            let right = Box::new(self.term());
            expr = expr::Expr::Binary {
                left: Box::new(expr),
                operator,
                right
            };
        }

        expr

    }

    fn term(&mut self) -> expr::Expr {

        let mut expr = self.factor();

        while self.rmatch(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = Box::new(self.factor());
            expr = expr::Expr::Binary {
                left: Box::new(expr),
                operator,
                right
            }
        }

        expr

    }

    fn factor(&mut self) -> expr::Expr {

        let mut expr = self.unary();

        while self.rmatch(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = Box::new(self.unary());
            expr = expr::Expr::Binary {
                left: Box::new(expr),
                operator,
                right
            }
        }

        expr

    }

    fn unary(&mut self) -> expr::Expr {

        if self.rmatch(&[TokenType::Bang, TokenType::Minus]) {
            let operator= self.previous().clone();
            let right = Box::new(self.unary());
            expr::Expr::Unary {
                operator,
                right
            }
        }
        else {
            self.primray()
        }

    }

    fn primray(&mut self) -> expr::Expr {

        if self.rmatch(&[TokenType::False]) {
            return expr::Expr::Literal{ value: Object::Bool(false) }
        }

        if self.rmatch(&[TokenType::True]) {
            return expr::Expr::Literal{ value: Object::Bool(true) }
        }

        if self.rmatch(&[TokenType::Non]) {
            return expr::Expr::Literal { value: Object::Non }
        }

        if self.rmatch(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected ) after expression");
            return expr::Expr::Grouping { expression: Box::new(expr) }
        }

        if self.rmatch(&[TokenType::Number, TokenType::String]) {
            return expr::Expr::Literal { value: self.previous().clone().literal }
        }

        // Just for now
        panic!("ARRRRRRRR")

    }

    fn rmatch(&mut self, types: &[TokenType]) -> bool {

        for ty in types {
            if self.check(ty) {
                self.advance();
                return true;
            }
        }
        false

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
