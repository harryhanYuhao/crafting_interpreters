use std::collections::{HashSet, HashMap};
use std::fmt;

// Grammar of the token: see docs/grammar.md
#[allow(dead_code, unused_variables, non_camel_case_types)]
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
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    //One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,
    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,
    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
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
            (TokenType::PLUS, "PLUS"),
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
            (TokenType::FUN, "FUN"),
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

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match DEBUG_STRING.get(self) {
            Some(s) => write!(f, "{}", s),
            None => write!(f, "UNKNOWN"),
        }
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    // literal: String,
    pub line: u32,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}   {:?}; l#: {}",
            self.lexeme, self.token_type, self.line
        )
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

    pub fn is_terminal(&self) -> bool {
        if TERMINAL_SET.contains(&self.token_type) {
            return true;
        }
        false
    }
}
