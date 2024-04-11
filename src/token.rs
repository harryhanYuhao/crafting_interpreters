use std::fmt::{self, Display};

#[allow(dead_code, unused_variables, non_camel_case_types)]
#[derive(Clone, Copy)]
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

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tmp = String::new();
        match self {
            TokenType::LEFT_PAREN => tmp.push_str("LEFT_PAREN"),
            TokenType::RIGHT_PAREN => tmp.push_str("RIGHT_PAREN"),
            TokenType::LEFT_BRACE => tmp.push_str("LEFT_BRACE"),
            TokenType::RIGHT_BRACE => tmp.push_str("RIGHT_BRACE"),
            TokenType::LEFT_BRACKET => tmp.push_str("LEFT_BRAKET"),
            TokenType::RIGHT_BRACKET => tmp.push_str("RIGHT_BRAKET"),
            TokenType::COMMA => tmp.push_str("COMMA"),
            TokenType::DOT => tmp.push_str("DOT"),
            TokenType::MINUS => tmp.push_str("MINUS"),
            TokenType::PLUS => tmp.push_str("PLUS"),
            TokenType::SEMICOLON => tmp.push_str("SEMICOLON"),
            TokenType::SLASH => tmp.push_str("SLASH"),
            TokenType::STAR => tmp.push_str("STAR"),
            TokenType::BANG => tmp.push_str("BANG"),
            TokenType::BANG_EQUAL => tmp.push_str("BANG_EQUAL"),
            TokenType::EQUAL => tmp.push_str("EQUAL"),
            TokenType::EQUAL_EQUAL => tmp.push_str("EQUAL_EQUAL"),
            TokenType::GREATER => tmp.push_str("GREATER"),
            TokenType::GREATER_EQUAL => tmp.push_str("GREATER_EQUAL"),
            TokenType::LESS => tmp.push_str("LESS"),
            TokenType::LESS_EQUAL => tmp.push_str("LESS_EQUAL"),
            TokenType::IDENTIFIER => tmp.push_str("IDENTIFIER"),
            TokenType::STRING => tmp.push_str("STRING"),
            TokenType::NUMBER => tmp.push_str("NUMBER"),
            TokenType::AND => tmp.push_str("AND"),
            TokenType::CLASS => tmp.push_str("CLASS"),
            TokenType::ELSE => tmp.push_str("ELSE"),
            TokenType::FALSE => tmp.push_str("FALSE"),
            TokenType::FUN => tmp.push_str("FUN"),
            TokenType::FOR => tmp.push_str("FOR"),
            TokenType::IF => tmp.push_str("IF"),
            TokenType::NIL => tmp.push_str("NIL"),
            TokenType::OR => tmp.push_str("OR"),
            TokenType::PRINT => tmp.push_str("PRINT"),
            TokenType::RETURN => tmp.push_str("RETURN"),
            TokenType::SUPER => tmp.push_str("SUPER"),
            TokenType::THIS => tmp.push_str("THIS"),
            TokenType::TRUE => tmp.push_str("TRUE"),
            TokenType::VAR => tmp.push_str("VAR"),
            TokenType::WHILE => tmp.push_str("WHILE"),
            TokenType::EOF => tmp.push_str("EOF"),
        }
        write!(f, "{}", tmp)
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    // literal: String,
    line: u32,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}; {}; l#: {}", self.token_type, self.lexeme, self.line)
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
}
