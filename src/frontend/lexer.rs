use std::process;

use super::token::{ Token, TokenType, Object };

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

    pub fn scan_tokens(&mut self) -> &Vec<Token> {


        while !self.is_end() {

            self.start = self.current;
            self.scan_token();

        }

        self.tokens.push(
            Token::new(
                TokenType::Eof,
                String::from(""),
                Object::Non,
                self.line
            )
        );

        &self.tokens

    }

    fn scan_token(&mut self) {

        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual)
                }
                else {
                    self.add_token(TokenType::Bang)
                }
            },
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual)
                }
                else {
                    self.add_token(TokenType::Equal)
                }
            },
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                }
                else {
                    self.add_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual)
                }
                else {
                    self.add_token(TokenType::Less)
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        let _ = self.advance();
                    }
                }
                else {
                    self.add_token(TokenType::Slash)
                }
            }
            _ => {
                if c != '\n' {
                    println!("Unexpected character at -> {}:{}", self.line, self.current);
                    process::exit(65)
                }
            }
        };

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

    fn advance(&mut self) -> char {

        let c = self.source[self.current..].chars().next().unwrap();
        self.current += 1;
        c

    }

    fn add_token(&mut self, token_type: TokenType) {

        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(
                token_type,
                text,
                Object::Non,
                self.line
        );

        self.tokens.push(token);

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
