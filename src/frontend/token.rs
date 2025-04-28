use std::fmt;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {

    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two characters token
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fn, For, If, Non, Or,
    Print, Return, Base, This, True, Let, While,
    Eof

}
pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {

    let mut map = HashMap::new();
    map.insert("and", TokenType::And);
    map.insert("class", TokenType::Class);
    map.insert("else", TokenType::Else);
    map.insert("false", TokenType::False);
    map.insert("for", TokenType::For);
    map.insert("fn", TokenType::Fn);
    map.insert("if", TokenType::If);
    map.insert("non", TokenType::Non);
    map.insert("or", TokenType::Or);
    map.insert("print", TokenType::Print);
    map.insert("return", TokenType::Return);
    map.insert("base", TokenType::Base);
    map.insert("this", TokenType::This);
    map.insert("true", TokenType::True);
    map.insert("let", TokenType::Let);
    map.insert("while", TokenType::While);
    map

});

pub enum Object {

    Number(f64),
    Str(String),
    Bool(bool),
    Non

}

impl fmt::Display for Object {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Number(n) => write!(f, "{}", n),
            Object::Str(s) => write!(f, "{}", s),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Non => write!(f, "Non"),
        }
    }

}

pub struct Token {

    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: usize

}

impl fmt::Display for TokenType {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let text = match self {
            // Single-character tokens
            TokenType::LeftParen => "Left Paren",
            TokenType::RightParen => "Right Paren",
            TokenType::LeftBrace => "Left Brace",
            TokenType::RightBrace => "Right Brace",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Minus => "Minus",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Slash => "Slash",
            TokenType::Star => "Star",

            // One or two character tokens
            TokenType::Bang => "Bang",
            TokenType::BangEqual => "Bang Equal",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "Equal Equal",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "Greater Equal",
            TokenType::Less => "Less",
            TokenType::LessEqual => "Less Equal",

            // Literals
            TokenType::Identifier => "Identifier",
            TokenType::String => "String",
            TokenType::Number => "Number",

            // Keywords
            TokenType::And => "IDENTIFIER(and)",
            TokenType::Class => "Class",
            TokenType::Else => "Else",
            TokenType::False => "Fasle",
            TokenType::Fn => "Function",
            TokenType::For => "For",
            TokenType::If => "If",
            TokenType::Non => "Non",
            TokenType::Or => "Or",
            TokenType::Print => "Print",
            TokenType::Return => "Return",
            TokenType::Base => "Base",
            TokenType::This => "This",
            TokenType::True => "True",
            TokenType::Let => "IDENTIFIER(let)",
            TokenType::While => "While",

            TokenType::Eof => "End of File",
        };
        write!(f, "{text}")

    }

}

impl Token {

    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) -> Self {

        Token{
            token_type,
            lexeme,
            literal,
            line
        }

    }

    pub fn show(&self) {
        println!("{} - {} - {}", self.token_type, self.lexeme, self.literal);
    }

}
