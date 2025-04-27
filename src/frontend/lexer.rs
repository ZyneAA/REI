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
            '(' => self.add_token(TokenType::LeftParen, Object::Non),
            ')' => self.add_token(TokenType::RightParen, Object::Non),
            '{' => self.add_token(TokenType::LeftBrace, Object::Non),
            '}' => self.add_token(TokenType::RightBrace, Object::Non),
            ',' => self.add_token(TokenType::Comma, Object::Non),
            '.' => self.add_token(TokenType::Dot, Object::Non),
            '-' => self.add_token(TokenType::Minus, Object::Non),
            '+' => self.add_token(TokenType::Plus, Object::Non),
            ';' => self.add_token(TokenType::Semicolon, Object::Non),
            '*' => self.add_token(TokenType::Star, Object::Non),
            '!' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::BangEqual, Object::Non)
                }
                else {
                    self.add_token(TokenType::Bang, Object::Non)
                }
            },
            '=' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::EqualEqual, Object::Non)
                }
                else {
                    self.add_token(TokenType::Equal, Object::Non)
                }
            },
            '>' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::GreaterEqual, Object::Non)
                }
                else {
                    self.add_token(TokenType::Greater, Object::Non)
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    self.add_token(TokenType::LessEqual, Object::Non)
                }
                else {
                    self.add_token(TokenType::Less, Object::Non)
                }
            }
            '/' => {
                if self.match_next_char('/') {
                    while self.peek() != '\n' && !self.is_end() {
                        let _ = self.advance();
                    }
                }
                else {
                    self.add_token(TokenType::Slash, Object::Non)
                }
            }
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if c != '\n' {
                    println!("Unexpected character at -> {}:{}", self.line, self.current);
                    process::exit(65)
                }
            }
        };

    }

    fn string(&mut self) {

        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
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

    fn advance(&mut self) -> char {

        let c = self.source[self.current..].chars().next().unwrap();
        self.current += 1;
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
