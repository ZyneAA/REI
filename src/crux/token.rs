use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::backend::rei_callable::ReiCallable;
use crate::backend::rei_instance::ReiInstance;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    DotDot,
    Minus,
    Plus,
    Semicolon,
    Fullcolon,
    Slash,
    Star,

    // One or two characters token
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,
    Range,

    // Keywords
    And,
    Class,
    Setter,
    Getter,
    Static,
    Else,
    False,
    Fn,
    For,
    If,
    Null,
    Or,
    At,
    Print,
    PrintLn,
    Return,
    Base,
    This,
    True,
    Let,
    While,
    Loop,
    Break,
    Continue,
    Throw,
    Do,
    Fail,
    Yield,
    Underscore,
    Eof,

    // Module related
    Use,
    Expose,
    As,
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
    map.insert("null", TokenType::Null);
    map.insert("or", TokenType::Or);
    map.insert("do", TokenType::Do);
    map.insert("fail", TokenType::Fail);
    map.insert("yield", TokenType::Fail);
    map.insert("print", TokenType::Print);
    map.insert("println", TokenType::PrintLn);
    map.insert("return", TokenType::Return);
    map.insert("base", TokenType::Base);
    map.insert("this", TokenType::This);
    map.insert("static", TokenType::Static);
    map.insert("true", TokenType::True);
    map.insert("let", TokenType::Let);
    map.insert("while", TokenType::While);
    map.insert("loop", TokenType::Loop);
    map.insert("break", TokenType::Break);
    map.insert("throw", TokenType::Throw);
    map.insert("_", TokenType::Underscore);
    map.insert("use", TokenType::Use);
    map.insert("expose", TokenType::Expose);
    map.insert("as", TokenType::As);
    map.insert("@", TokenType::At);
    map.insert("continue", TokenType::Continue);

    map
});

#[derive(Clone, Debug)]
pub enum Object {
    Number(f64),
    Bool(bool),
    Range(f64, f64),
    Str(String),
    Dummy,
    Null,
    Callable(Rc<dyn ReiCallable>),
    Instance(Rc<RefCell<ReiInstance>>),
    MBlock(*mut u8, usize),
    Vec(Rc<RefCell<Vec<Object>>>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Number(n) => write!(f, "{}", n),
            Object::Str(s) => write!(f, "{}", s),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Range(s, e) => write!(f, "{}:{}", s, e),
            Object::Dummy => write!(f, "Dummy"),
            Object::Callable(c) => write!(f, "{}", c.to_string()),
            Object::Instance(i) => write!(f, "{}", i.borrow().to_string()),
            Object::MBlock(p, s) => write!(f, "{:p} {}", p, s),
            Object::Null => write!(f, "Null"),
            Object::Vec(v) => {
                let vec_borrow = v.borrow();
                let elements: Vec<String> = vec_borrow.iter().map(|o| o.to_string()).collect();
                write!(f, "[{}]", elements.join(", "))
            }
        }
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
    pub place: usize,
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
            TokenType::DotDot => "DotDot",
            TokenType::Minus => "Minus",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Fullcolon => "Fullcolon",
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
            TokenType::Getter => "Getter",
            TokenType::Setter => "Setter",

            // Literals
            TokenType::Identifier => "Identifier",
            TokenType::String => "STRING",
            TokenType::Number => "NUMBER",
            TokenType::Range => "RANGE",

            // Keywords
            TokenType::And => "IDENTIFIER",
            TokenType::Class => "IDENTIFIER",
            TokenType::Else => "IDENTIFIER",
            TokenType::False => "IDENTIFIER",
            TokenType::Fn => "IDENTIFIER",
            TokenType::For => "IDENTIFIER",
            TokenType::If => "IDENTIFIER",
            TokenType::Null => "IDENTIFIER",
            TokenType::Or => "IDENTIFIER",
            TokenType::Do => "IDENTIFIER",
            TokenType::Fail => "IDENTIFIER",
            TokenType::Yield => "IDENTIFIER",
            TokenType::Print => "IDENTIFIER",
            TokenType::PrintLn => "IDENTIFIER",
            TokenType::Return => "IDENTIFIER",
            TokenType::Base => "IDENTIFIER",
            TokenType::This => "IDENTIFIER",
            TokenType::Static => "IDENTIFIER",
            TokenType::True => "IDENTIFIER",
            TokenType::Let => "IDENTIFIER",
            TokenType::Loop => "IDENTIFIER",
            TokenType::While => "IDENTIFIER",
            TokenType::Break => "IDENTIFIER",
            TokenType::Continue => "IDENTIFIER",
            TokenType::Use => "IDENTIFIER",
            TokenType::Throw => "IDENTIFIER",
            TokenType::Underscore => "IDENTIFIER",
            TokenType::Expose => "IDENTIFIER",
            TokenType::As => "IDENTIFIER",
            TokenType::At => "IDENTIFIER",

            TokenType::Eof => "End of File",
        };

        write!(f, "{text}")
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Object,
        line: usize,
        place: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            place,
        }
    }

    pub fn fake(token_type: TokenType) -> Self {
        Token {
            token_type,
            lexeme: format!("{:?}", token_type),
            literal: Object::Dummy,
            line: 0,
            place: 0,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.literal {
            Object::Null => write!(
                f,
                "{} -->'{}'<-- at {}:{}",
                self.token_type, self.lexeme, self.line, self.place
            ),
            _ => write!(
                f,
                "{} -->' {} '<-- {} at : {}:{}",
                self.token_type, self.lexeme, self.literal, self.line, self.place
            ),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("token_type", &self.token_type)
            .field("lexeme", &self.lexeme)
            .field("literal", &self.literal)
            .field("line", &self.line)
            .finish()
    }
}
