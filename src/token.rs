use rand::prelude::*;
use std::collections::HashMap;
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
    STMT_SEP, // statement separator, ; and new line
    SLASH,    // copulative
    SLASH_EQUAL,
    STAR,     // copulative
    STAR_EQUAL,
    PERCENT,  // copulative
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
    // Dummy Token
    DUMMY
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LEFT_PAREN => write!(f, "LEFT_PAREN"),
            TokenType::RIGHT_PAREN => write!(f, "RIGHT_PAREN"),
            TokenType::LEFT_BRACE => write!(f, "LEFT_BRACE"),
            TokenType::RIGHT_BRACE => write!(f, "RIGHT_BRACE"),
            TokenType::LEFT_BRACKET => write!(f, "LEFT_BRACKET"),
            TokenType::RIGHT_BRACKET => write!(f, "RIGHT_BRACKET"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::DOT => write!(f, "DOT"),
            TokenType::STMT_SEP => write!(f, "STMT_SEP"),
            TokenType::SLASH => write!(f, "SLASH"),
            TokenType::SLASH_EQUAL => write!(f, "SLASH_EQUAL"),
            TokenType::STAR => write!(f, "STAR"),
            TokenType::STAR_EQUAL => write!(f, "STAR_EQUAL"),
            TokenType::PERCENT => write!(f, "PERCENT"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::PLUS_EQUAL => write!(f, "PLUS_EQUAL"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::MINUS_EQUAL => write!(f, "MINUS_EQUAL"),
            TokenType::BANG => write!(f, "BANG"),
            TokenType::BANG_EQUAL => write!(f, "BANG_EQUAL"),
            TokenType::EQUAL => write!(f, "EQUAL"),
            TokenType::EQUAL_EQUAL => write!(f, "EQUAL_EQUAL"),
            TokenType::GREATER => write!(f, "GREATER"),
            TokenType::GREATER_EQUAL => write!(f, "GREATER_EQUAL"),
            TokenType::LESS => write!(f, "LESS"),
            TokenType::LESS_EQUAL => write!(f, "LESS_EQUAL"),
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::STRING => write!(f, "STRING"),
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::AND => write!(f, "AND"),
            TokenType::CLASS => write!(f, "CLASS"),
            TokenType::ELSE => write!(f, "ELSE"),
            TokenType::FALSE => write!(f, "FALSE"),
            TokenType::FN => write!(f, "FN"),
            TokenType::FOR => write!(f, "FOR"),
            TokenType::IF => write!(f, "IF"),
            TokenType::NIL => write!(f, "NIL"),
            TokenType::OR => write!(f, "OR"),
            TokenType::PRINT => write!(f, "PRINT"),
            TokenType::RETURN => write!(f, "RETURN"),
            TokenType::SUPER => write!(f, "SUPER"),
            TokenType::THIS => write!(f, "THIS"),
            TokenType::TRUE => write!(f, "TRUE"),
            TokenType::VAR => write!(f, "VAR"),
            TokenType::WHILE => write!(f, "WHILE"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::DUMMY => write!(f, "DUMMY"),
        }
    }
}

// This list only used for generating random TokenType from index
static TOKEN_TYPE_LIST: [TokenType; 46] = [
    TokenType::LEFT_PAREN,
    TokenType::RIGHT_PAREN,
    TokenType::LEFT_BRACE,
    TokenType::RIGHT_BRACE,
    TokenType::LEFT_BRACKET,
    TokenType::RIGHT_BRACKET,
    TokenType::COMMA,
    TokenType::DOT,
    TokenType::STMT_SEP,
    TokenType::SLASH,   // copulative
    TokenType::SLASH_EQUAL,   // copulative
    TokenType::STAR,    // copulative
    TokenType::STAR_EQUAL,    // copulative
    TokenType::PERCENT, // copulative
    TokenType::PLUS,
    TokenType::PLUS_EQUAL,
    TokenType::MINUS, // copulative
    TokenType::MINUS_EQUAL,
    TokenType::BANG,
    TokenType::BANG_EQUAL,
    TokenType::EQUAL,
    TokenType::EQUAL_EQUAL,   //copulative
    TokenType::GREATER,       // copulative
    TokenType::GREATER_EQUAL, // copulative
    TokenType::LESS,          // copulative
    TokenType::LESS_EQUAL,    // copulative
    TokenType::IDENTIFIER,
    TokenType::STRING,
    TokenType::NUMBER,
    TokenType::CLASS,
    TokenType::ELSE,
    TokenType::FALSE,
    TokenType::FN,
    TokenType::FOR,
    TokenType::IF,
    TokenType::NIL,
    TokenType::OR, // copulative
    TokenType::PRINT,
    TokenType::RETURN,
    TokenType::SUPER,
    TokenType::THIS,
    TokenType::TRUE,
    TokenType::VAR,
    TokenType::WHILE,
    TokenType::EOF,
    TokenType::DUMMY,
];

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

impl TokenType {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..TOKEN_TYPE_LIST.len());
        TOKEN_TYPE_LIST[n].clone()
    }
}

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize, // starts from 1, the line number of the token
    pub column: usize, // starts from 1, the column number of the token
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:<4} {:?} {}:{})",
            self.lexeme, self.token_type, self.line, self.column
        )
    }
}


impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }
    
    pub fn dummy() -> Self {
        Token{
            token_type: TokenType::DUMMY,
            lexeme: String::new(),
            line: 0,
            column: 0
        }
    }

    pub fn random() -> Self {
        Token {
            token_type: TokenType::random(),
            lexeme: String::from("DUMMY"),
            line: 0,
            column: 0,
        }
    }

    pub(crate) fn get_token_type_from_arc(input: Arc<Mutex<Token>>) -> TokenType {
        let token = input.lock().unwrap();
        token.token_type
    }
}

impl Into<Arc<Mutex<Token>>> for Token {
    fn into(self) -> Arc<Mutex<Token>> {
        Arc::new(Mutex::new(self))
    }
}

pub type TokenArcVec = Vec<Arc<Mutex<Token>>>;
