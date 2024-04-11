use crate::token::{Token, TokenType};
use std::collections::HashMap;
use std::error::Error;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".into(), TokenType::AND);
        m.insert("class".into(), TokenType::CLASS);
        m.insert("else".into(), TokenType::ELSE);
        m.insert("false".into(), TokenType::FALSE);
        m.insert("fun".into(), TokenType::FUN);
        m.insert("for".into(), TokenType::FOR);
        m.insert("if".into(), TokenType::IF);
        m.insert("nil".into(), TokenType::NIL);
        m.insert("or".into(), TokenType::OR);
        m.insert("print".into(), TokenType::PRINT);
        m.insert("return".into(), TokenType::RETURN);
        m.insert("super".into(), TokenType::SUPER);
        m.insert("this".into(), TokenType::THIS);
        m.insert("ture".into(), TokenType::TRUE);
        m.insert("var".into(), TokenType::VAR);
        m.insert("while".into(), TokenType::WHILE);
        m.insert("EOF".into(), TokenType::EOF);
        m
    };
}

pub fn scan_tokens(source: &str, line: &mut u32) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut token_vec = Vec::new();
    let mut start: usize = 0;
    let mut current: usize = 0;
    let source_vec = source.chars().collect::<Vec<char>>();
    let num_of_chars = source_vec.len();

    while current < num_of_chars {
        start = current;
        if let Some(token) = scan_iteration(&source_vec, start, &mut current, line) {
            token_vec.push(token);
        }
    }
    Ok(token_vec)
}

fn scan_iteration(
    source_vec: &Vec<char>,
    start: usize,
    current: &mut usize,
    line: &mut u32,
) -> Option<Token> {
    let mut poke = start + 1;
    let token: Token;
    match source_vec[start] {
        '(' => token = Token::new(TokenType::LEFT_PAREN, String::from('('), *line),
        ')' => token = Token::new(TokenType::RIGHT_PAREN, String::from(')'), *line),
        '[' => {
            token = Token::new(
                TokenType::LEFT_BRACKET,
                String::from(source_vec[start]),
                *line,
            )
        }
        ']' => {
            token = Token::new(
                TokenType::RIGHT_BRACKET,
                String::from(source_vec[start]),
                *line,
            )
        }
        '}' => token = Token::new(TokenType::LEFT_BRACE, String::from(source_vec[start]), *line),
        '{' => {
            token = Token::new(
                TokenType::RIGHT_BRACE,
                String::from(source_vec[start]),
                *line,
            )
        }
        ',' => token = Token::new(TokenType::COMMA, String::from(source_vec[start]), *line),
        '.' => token = Token::new(TokenType::DOT, String::from(source_vec[start]), *line),
        '-' => token = Token::new(TokenType::MINUS, String::from(source_vec[start]), *line),
        '+' => token = Token::new(TokenType::PLUS, String::from(source_vec[start]), *line),
        ';' => token = Token::new(TokenType::SEMICOLON, String::from(source_vec[start]), *line),
        '*' => token = Token::new(TokenType::STAR, String::from(source_vec[start]), *line),
        '!' => {
            if source_vec[poke] == '=' {
                poke += 1;
                let tmp = get_string(start, poke, source_vec);
                token = Token::new(TokenType::BANG_EQUAL, tmp, *line);
            } else {
                token = Token::new(TokenType::BANG, String::from(source_vec[start]), *line);
            }
        }
        '=' => {
            if source_vec[poke] == '=' {
                poke += 1;
                let tmp = get_string(start, poke, source_vec);
                token = Token::new(TokenType::EQUAL_EQUAL, tmp, *line);
            } else {
                token = Token::new(TokenType::EQUAL, String::from(source_vec[start]), *line);
            }
        }
        '>' => {
            if source_vec[poke] == '=' {
                poke += 1;
                let tmp = get_string(start, poke, source_vec);
                token = Token::new(TokenType::GREATER_EQUAL, tmp, *line);
            } else {
                token = Token::new(TokenType::GREATER, String::from(source_vec[start]), *line);
            }
        }
        '<' => {
            if source_vec[poke] == '=' {
                poke += 1;
                let tmp = get_string(start, poke, source_vec);
                token = Token::new(TokenType::LESS_EQUAL, tmp, *line);
            } else {
                token = Token::new(TokenType::LESS, String::from(source_vec[start]), *line);
            }
        }
        '"' => {
            while source_vec[poke] != '"' {
                if poke >= source_vec.len() {
                    panic!("unterminated string!");
                }
                poke += 1;
            }
            poke += 1;
            let tmp = get_string(start + 1, poke - 1, source_vec);
            token = Token::new(TokenType::STRING, tmp, *line);
        }
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
            while source_vec[poke].is_digit(10) && poke < source_vec.len() {
                poke += 1;
            }
            if poke < source_vec.len() {
                if source_vec[poke] == '.' {
                    poke += 1;
                    while source_vec[poke].is_digit(10) && poke < source_vec.len() {
                        poke += 1;
                    }
                }
            }
            if source_vec[poke - 1] == '.' {
                let tmp = get_string(start, poke - 1, source_vec);
                token = Token::new(TokenType::NUMBER, tmp, *line);
            } else {
                let tmp = get_string(start, poke, source_vec);
                token = Token::new(TokenType::NUMBER, tmp, *line);
            }
        }
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o'
        | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | 'A' | 'B' | 'C'
        | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q'
        | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' => {
            while source_vec[poke].is_alphanumeric() && source_vec[poke].is_ascii()
                || source_vec[poke] == '_' && poke < source_vec.len()
            {
                poke += 1;
            }
            let tmp = get_string(start, poke, source_vec);
            let mut token_type: TokenType = TokenType::IDENTIFIER;
            if let Some(tmp) = KEYWORDS.get(&tmp) {
                token_type = (*tmp).clone();
            }
            token = Token::new(token_type, tmp, *line);
        }

        // ignore
        ' ' => {
            *current = poke;
            return None;
        }
        '\n' => {
            *current = poke;
            *line += 1;
            return None;
        }
        '/' => {
            if source_vec[poke] == '/' {
                *current = source_vec.len();
                return None;
            }
            token = Token::new(TokenType::SLASH, String::from(source_vec[start]), *line)
        }
        _ => panic!("Invalid Token at line {}", *line),
    }

    *current = poke;
    Some(token)
}

fn get_string(start: usize, end: usize, char_vec: &Vec<char>) -> String {
    let mut tmp = String::new();
    for i in start..end {
        tmp.push(char_vec[i]);
    }
    tmp
}
