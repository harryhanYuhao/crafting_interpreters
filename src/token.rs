use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::convert::{From, Into};
use std::fmt;
use std::sync::{Arc, Mutex};

// Grammar of the token: see docs/grammar.md
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_BRACKET,
    RIGHT_BRACKET,
    COMMA,
    DOT,
    SEMICOLON,
    SLASH, // copulative
    STAR,  // copulative
    //One or two character tokens.
    PLUS,
    PLUS_EQUAL,
    MINUS, // copulative
    MINUS_EQUAL,
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,   //copulative
    GREATER,       // copulative
    GREATER_EQUAL, // copulative
    LESS,          // copulative
    LESS_EQUAL,    // copulative
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND, // copulative
    CLASS,
    ELSE,
    FALSE,
    FN,
    FOR,
    IF,
    NIL,
    OR, // copulative
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

// TODO: REMOVE THESE HASHMAPS

lazy_static! {
    static ref NUMBERRED_TB: HashMap<usize, TokenType> = {
        HashMap::from([
            (0, TokenType::LEFT_PAREN),
            (1, TokenType::RIGHT_PAREN),
            (2, TokenType::LEFT_BRACE),
            (3, TokenType::RIGHT_BRACE),
            (4, TokenType::LEFT_BRACKET),
            (5, TokenType::RIGHT_BRACKET),
            (6, TokenType::COMMA),
            (7, TokenType::DOT),
            (8, TokenType::MINUS),
            (9, TokenType::PLUS),
            (10, TokenType::SEMICOLON),
            (11, TokenType::SLASH),
            (12, TokenType::STAR),
            (13, TokenType::BANG),
            (14, TokenType::BANG_EQUAL),
            (15, TokenType::EQUAL),
            (16, TokenType::EQUAL_EQUAL),
            (17, TokenType::GREATER),
            (18, TokenType::GREATER_EQUAL),
            (19, TokenType::LESS),
            (20, TokenType::LESS_EQUAL),
            (21, TokenType::IDENTIFIER),
            (22, TokenType::STRING),
            (23, TokenType::NUMBER),
            (24, TokenType::AND),
            (25, TokenType::CLASS),
            (26, TokenType::ELSE),
            (27, TokenType::FALSE),
            (28, TokenType::FN),
            (29, TokenType::FOR),
            (30, TokenType::IF),
            (31, TokenType::NIL),
            (32, TokenType::OR),
            (33, TokenType::PRINT),
            (34, TokenType::RETURN),
            (35, TokenType::SUPER),
            (36, TokenType::THIS),
            (37, TokenType::TRUE),
            (38, TokenType::VAR),
            (39, TokenType::WHILE),
            (40, TokenType::EOF),
        ])
    };
}

lazy_static! {
    // if a type is in this set, it is a terminal
    static ref TERMINAL_SET: HashSet<TokenType> = {
        HashSet::from([
            TokenType::NUMBER,
            TokenType::EOF,
            TokenType::TRUE,
            TokenType::FALSE,
        ])
    };
}

lazy_static! {
    static ref DEBUG_STRING: HashMap<TokenType, &'static str> = {
        HashMap::from([
            (TokenType::LEFT_PAREN, "LEFT_PAREN"),
            (TokenType::RIGHT_PAREN, "RIGHT_PAREN"),
            (TokenType::LEFT_BRACE, "LEFT_BRACE"),
            (TokenType::RIGHT_BRACE, "RIGHT_BRACE"),
            (TokenType::LEFT_BRACKET, "LEFT_BRACKET"),
            (TokenType::RIGHT_BRACKET, "RIGHT_BRACKET"),
            (TokenType::COMMA, "COMMA"),
            (TokenType::DOT, "DOT"),
            (TokenType::MINUS, "MINUS"),
            (TokenType::MINUS_EQUAL, "MINUS_EQUAL"),
            (TokenType::PLUS, "PLUS"),
            (TokenType::PLUS_EQUAL, "PLUS_EQUAL"),
            (TokenType::SEMICOLON, "SEMICOLON"),
            (TokenType::SLASH, "SLASH"),
            (TokenType::STAR, "STAR"),
            (TokenType::BANG, "BANG"),
            (TokenType::BANG_EQUAL, "BANG_EQUAL"),
            (TokenType::EQUAL, "EQUAL"),
            (TokenType::EQUAL_EQUAL, "EQUAL_EQUAL"),
            (TokenType::GREATER, "GREATER"),
            (TokenType::GREATER_EQUAL, "GREATER_EQUAL"),
            (TokenType::LESS, "LESS"),
            (TokenType::LESS_EQUAL, "LESS_EQUAL"),
            (TokenType::IDENTIFIER, "IDENTIFIER"),
            (TokenType::STRING, "STRING"),
            (TokenType::NUMBER, "NUMBER"),
            (TokenType::AND, "AND"),
            (TokenType::CLASS, "CLASS"),
            (TokenType::ELSE, "ELSE"),
            (TokenType::FALSE, "FALSE"),
            (TokenType::FN, "FN"),
            (TokenType::FOR, "FOR"),
            (TokenType::IF, "IF"),
            (TokenType::NIL, "NIL"),
            (TokenType::OR, "OR"),
            (TokenType::PRINT, "PRINT"),
            (TokenType::RETURN, "RETURN"),
            (TokenType::SUPER, "SUPER"),
            (TokenType::THIS, "THIS"),
            (TokenType::TRUE, "TRUE"),
            (TokenType::VAR, "VAR"),
            (TokenType::WHILE, "WHILE"),
            (TokenType::EOF, "EOF"),
        ])
    };
}

lazy_static! {
    pub static ref KEYWORDS_TO_TOKEN: HashMap<String, TokenType> = {
        HashMap::from([
            ("and".into(), TokenType::AND),
            ("class".into(), TokenType::CLASS),
            ("else".into(), TokenType::ELSE),
            ("false".into(), TokenType::FALSE),
            ("fn".into(), TokenType::FN),
            ("for".into(), TokenType::FOR),
            ("if".into(), TokenType::IF),
            ("nil".into(), TokenType::NIL),
            ("or".into(), TokenType::OR),
            ("print".into(), TokenType::PRINT),
            ("return".into(), TokenType::RETURN),
            ("super".into(), TokenType::SUPER),
            ("this".into(), TokenType::THIS),
            ("true".into(), TokenType::TRUE),
            ("var".into(), TokenType::VAR),
            ("while".into(), TokenType::WHILE),
            ("EOF".into(), TokenType::EOF),
        ])
    };
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match DEBUG_STRING.get(self) {
            Some(s) => write!(f, "{}", s),
            None => write!(f, "UNKNOWN_TYPE FOR DEBUG TRAIT"),
        }
    }
}

impl TokenType {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..NUMBERRED_TB.len());
        *NUMBERRED_TB.get(&n).unwrap()
    }
}

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:<4}{:?})", self.lexeme, self.token_type)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn random() -> Self {
        Token {
            token_type: TokenType::random(),
            lexeme: String::from("DUMMY"),
            line: 0,
        }
    }

    pub fn is_terminal(&self) -> bool {
        if TERMINAL_SET.contains(&self.token_type) {
            return true;
        }
        false
    }
}

impl Into<Arc<Mutex<Token>>> for Token {
    fn into(self) -> Arc<Mutex<Token>> {
        Arc::new(Mutex::new(self))
    }
}

pub type TokenArcVec = Vec<Arc<Mutex<Token>>>;

pub fn get_token_type_from_arc(input: Arc<Mutex<Token>>) -> TokenType {
    let token = input.lock().unwrap();
    token.token_type
}
