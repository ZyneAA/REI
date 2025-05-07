use std::process;

use crate::crux::token::{ Token, TokenType, Object, KEYWORDS };
use crate::crux::error::{ ReiError, SyntaxError };

pub struct Lexer<'a> {

    source: &'a str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    length: usize

}

impl<'a> Lexer<'a> {

    pub fn new(source: &'a str) -> Self {

        let tokens: Vec<Token> = Vec::new();

        Lexer{
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
            length: source.len()
        }

    }

    pub fn scan_tokens(mut self) -> Vec<Token> {


        while !self.is_end() {

            self.start = self.current;
            self.scan_token();

        }

        self.tokens.push(
            Token::new(
                TokenType::Eof,
                String::from(""),
                Object::Null,
                self.line
            )
        );

        self.tokens

    }

    fn scan_token(&mut self) {

        let c = self.advance();
        match c {

            '(' => self.add_token(TokenType::LeftParen, Object::Null),
            ')' => self.add_token(TokenType::RightParen, Object::Null),
            '{' => self.add_token(TokenType::LeftBrace, Object::Null),
            '}' => self.add_token(TokenType::RightBrace, Object::Null),
            ',' => self.add_token(TokenType::Comma, Object::Null),
            '.' => self.add_token(TokenType::Dot, Object::Null),
            '-' => self.add_token(TokenType::Minus, Object::Null),
            '+' => self.add_token(TokenType::Plus, Object::Null),
            ';' => self.add_token(TokenType::Semicolon, Object::Null),
            '*' => self.add_token(TokenType::Star, Object::Null),
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual, Object::Null)
                }
                else {
                    self.add_token(TokenType::Bang, Object::Null)
                }
            },
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual, Object::Null)
                }
                else {
                    self.add_token(TokenType::Equal, Object::Null)
                }
            },
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual, Object::Null)
                }
                else {
                    self.add_token(TokenType::Greater, Object::Null)
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual, Object::Null)
                }
                else {
                    self.add_token(TokenType::Less, Object::Null)
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        let _ = self.advance();
                    }
                }
                else if self.match_next_char('*') {
                    while !self.is_end() {
                        if self.peek() == '*' && self.peek_next() == '/' {
                            // Skipping * and /
                            self.advance();
                            self.advance();
                            break;
                        }
                        if self.peek() == '\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }
                }
                else {
                    self.add_token(TokenType::Slash, Object::Null)
                }
            }
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {

                if c.is_digit(10) {
                    self.number();
                }
                else if self.is_alpha(c) {
                    self.identifier();
                }
                else {
                    SyntaxError::throw_error(&(self.line, self.current), "syntax error")
                }

            }

        };

    }

    fn number(&mut self) {

        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {

            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }

        }

        let number: f64 = self.source[self.start..self.current].to_string().parse().unwrap();
        self.add_token(TokenType::Number, Object::Number(number));

    }

    fn string(&mut self) {

        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            // Error handling needed
            println!("Unterminated string");
            process::exit(65);
        }

        self.advance();
        let sub_string = self.source[self.start + 1..self.current -1].to_string();
        self.add_token(TokenType::String, Object::Str(sub_string));

    }

    fn is_end(&self) -> bool {
        self.current >= self.length
    }

    fn peek(&mut self) -> char {

        if self.is_end() {
            '\0'
        }
        else {
            let c = self.source[self.current..].chars().next().unwrap();
            c
        }

    }

    fn peek_next(&mut self) -> char {

        if self.current + 1 >= self.length {
            return '\0'
        }

        let c = self.source[self.current + 1..].chars().next().unwrap();
        c

    }

    fn advance(&mut self) -> char {

        self.current += 1;
        let c = self.source[self.current - 1..].chars().next().unwrap();
        c

    }

    fn add_token(&mut self, token_type: TokenType, literal: Object) {

        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(
                token_type,
                text,
                literal,
                self.line
        );

        self.tokens.push(token);

    }

    fn identifier(&mut self) {

        let mut c = self.peek();
        while self.is_alpha_numeric(c) {
            self.advance();
            c = self.peek();
        }

        let token_type = match KEYWORDS.get(&self.source[self.start..self.current]){
            Some(v) => v.clone(),
            Nulle => TokenType::Identifier
        };

        self.add_token(token_type, Object::Null);

    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_digit(10)
    }

    fn match_next_char(&mut self, expected: char) -> bool {

        if self.is_end() {
            return false
        }

        let c = self.source[self.current..].chars().next().unwrap();
        if c != expected {
            return false
        }

        self.current += 1;
        true

    }

}
