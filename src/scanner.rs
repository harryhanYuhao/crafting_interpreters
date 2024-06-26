use crate::parser::{self};
use crate::token::{self, Token, TokenType};
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

pub fn token_vec_to_ls(in_vec: &[Arc<Mutex<Token>>]) -> Arc<Mutex<parser::LinkedTree>> {
    let ret = Arc::new(Mutex::new(parser::LinkedTree {
        tree: None,
        next: None,
    }));
    let mut ptr = Arc::clone(&ret);

    for (i, token) in in_vec.iter().enumerate() {
        if i == 0 {
            ptr.lock().unwrap().tree = Some(Arc::new(Mutex::new(
                parser::Tree::from_arc_mut_token(Arc::clone(token)),
            )));
            continue;
        }

        let next = parser::LinkedTree {
            tree: Some(Arc::new(Mutex::new(parser::Tree::from_arc_mut_token(
                Arc::clone(token),
            )))),
            next: None,
        };
        let next_ptr = Arc::new(Mutex::new(next));
        ptr.lock().unwrap().next = Some(Arc::clone(&next_ptr));
        ptr = Arc::clone(&next_ptr);
    }
    ret
}

pub fn scan_tokens(source: &str, line: &mut u32) -> Result<Vec<Arc<Mutex<Token>>>, Box<dyn Error>> {
    let mut token_vec: Vec<Arc<Mutex<Token>>> = Vec::new();
    let mut start: usize;
    let mut current: usize = 0;
    let source_vec = source.chars().collect::<Vec<char>>();
    let num_of_chars = source_vec.len();

    while current < num_of_chars {
        start = current;

        // scan iteration scans the text and return the next token wrapped in some.
        // If the next character does not constitute a token, it returns none
        // It increase the current counter per length of the character corresponds to the token
        if let Some(token) = scan_iteration(&source_vec, start, &mut current, line) {
            token_vec.push(Arc::new(Mutex::new(token)));
        }
    }
    Ok(token_vec)
}

// There are several kinds of tokens:
// One character tokens such as (, [, ], comma itself , ;
// Two character tokens such as ==, !=, <=, >=
// enclosing tokens such as ""
// multi character tokens such as identifiers and keywords
// We apply different scanning rules for each
// ////////////////////
// NOTE:
// This function scan and returns the next token, if plausible. Returns none is the next character does not constitutes a token, such as a new line or a comment
// Increase the current counter
fn scan_iteration(
    source_vec: &Vec<char>, // source
    start: usize, // the start of parsing: we are looking at source_vec[start] for the first
    // character
    current: &mut usize, // the current counter, shall be incremented per the length of the
    // char scanned
    line: &mut u32, // line number
) -> Option<Token> {
    // This function is to handle situations like this:
    // We have one character token, =, and two character token, ==
    // < and <=, > and >=, ! and !=, + and +=, - and -= The first character by iteself is a
    // valid token, but if it is followed by another specific charater, it constitutes a
    // different one. Note in lox there is not situation that three different token shares the
    // same first charater. (In C we have +, +=, and ++).
    fn two_character_check(
        poke: &mut usize,
        second_char: char,
        single_type: TokenType,
        combined_type: TokenType,
        line: u32,
        source_vec: &[char],
    ) -> Token {
        let token: Token;
        if source_vec.len() <= 1 {
            token = Token::new(single_type, String::from(source_vec[*poke - 1]), line);
        } else if source_vec[*poke] != second_char {
            token = Token::new(single_type, String::from(source_vec[*poke - 1]), line);
        } else {
            *poke += 1;
            token = Token::new(
                combined_type,
                source_vec[*poke - 2..=*poke - 1].iter().collect(),
                line,
            );
        }
        token
    }

    // START OF EXECUTION
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
        '}' => {
            token = Token::new(
                TokenType::LEFT_BRACE,
                String::from(source_vec[start]),
                *line,
            )
        }
        '{' => {
            token = Token::new(
                TokenType::RIGHT_BRACE,
                String::from(source_vec[start]),
                *line,
            )
        }
        ',' => token = Token::new(TokenType::COMMA, String::from(source_vec[start]), *line),
        '.' => token = Token::new(TokenType::DOT, String::from(source_vec[start]), *line),
        ';' => token = Token::new(TokenType::SEMICOLON, String::from(source_vec[start]), *line),
        '*' => token = Token::new(TokenType::STAR, String::from(source_vec[start]), *line),
        '-' => {
            token = two_character_check(
                &mut poke,
                '='.into(),
                TokenType::MINUS,
                TokenType::MINUS_EQUAL,
                *line,
                &source_vec,
            );
        }
        '+' => {
            token = two_character_check(
                &mut poke,
                '='.into(),
                TokenType::PLUS,
                TokenType::PLUS_EQUAL,
                *line,
                &source_vec,
            );
        }
        '!' => {
            token = two_character_check(
                &mut poke,
                '='.into(),
                TokenType::BANG,
                TokenType::BANG_EQUAL,
                *line,
                &source_vec,
            )
        }
        '=' => {
            token = two_character_check(
                &mut poke,
                '='.into(),
                TokenType::EQUAL,
                TokenType::EQUAL_EQUAL,
                *line,
                &source_vec,
            );
        }
        '>' => {
            token = two_character_check(
                &mut poke,
                '='.into(),
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
                *line,
                &source_vec,
            )
        }
        '<' => {
            token = two_character_check(
                &mut poke,
                '='.into(),
                TokenType::LESS,
                TokenType::LESS_EQUAL,
                *line,
                &source_vec,
            )
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
            // check decimal point
            while poke < source_vec.len() && source_vec[poke].is_digit(10) {
                poke += 1;
            }
            if poke < source_vec.len() && source_vec[poke] == '.' {
                poke += 1;
                while poke < source_vec.len() && source_vec[poke].is_digit(10) {
                    poke += 1;
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
            while (source_vec[poke].is_alphanumeric() || source_vec[poke] == '_')
                && poke < source_vec.len()
            {
                poke += 1;
            }
            let tmp = get_string(start, poke, source_vec);

            // identify if it is keyword
            let token_type: TokenType;
            if let Some(token) = token::KEYWORDS_TO_TOKEN.get(&tmp) {
                token_type = (*token).clone();
            } else {
                token_type = TokenType::IDENTIFIER;
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
                while source_vec[poke] != '\n' && poke < source_vec.len() {
                    poke += 1;
                }
                *current = poke;
                return None;
            }
            token = Token::new(TokenType::SLASH, String::from(source_vec[start]), *line)
        }
        _ => panic!(
            "{} is an invalid Token at line {}",
            source_vec[start], *line
        ),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_iteration() {
        let mut current = 0;
        let mut line = 1;
        let source = "!=".chars().collect::<Vec<char>>();
        let token = scan_iteration(&source, 0, &mut current, &mut line).unwrap();
        assert_eq!(token.token_type, TokenType::BANG_EQUAL);
        assert_eq!(token.lexeme, "!=");
        assert_eq!(token.line, 1);
    }

    #[test]
    fn test_scan_tokens() {
        fn test_helper(source: &str, expected: Vec<Token>) {
            let mut line = 0;
            let tokens = scan_tokens(source, &mut line).unwrap();
            assert_eq!(tokens.len(), expected.len());
            for (i, token) in tokens.iter().enumerate() {
                assert!(*token.lock().unwrap() == expected[i]);
            }
        }

        let source = r#"! != "hello" 123 123.123"#;
        let expected = vec![
            Token::new(TokenType::BANG, "!".into(), 0),
            Token::new(TokenType::BANG_EQUAL, "!=".into(), 0),
            Token::new(TokenType::STRING, "hello".into(), 0),
            Token::new(TokenType::NUMBER, "123".into(), 0),
            Token::new(TokenType::NUMBER, "123.123".into(), 0),
        ];
        test_helper(source, expected);

        let source = r#"1+2-3*4/5+1.22+5.22*21232.5347891"#;
        let expected = vec![
            Token::new(TokenType::NUMBER, "1".into(), 0),
            Token::new(TokenType::PLUS, "+".into(), 0),
            Token::new(TokenType::NUMBER, "2".into(), 0),
            Token::new(TokenType::MINUS, "-".into(), 0),
            Token::new(TokenType::NUMBER, "3".into(), 0),
            Token::new(TokenType::STAR, "*".into(), 0),
            Token::new(TokenType::NUMBER, "4".into(), 0),
            Token::new(TokenType::SLASH, "/".into(), 0),
            Token::new(TokenType::NUMBER, "5".into(), 0),
            Token::new(TokenType::PLUS, "+".into(), 0),
            Token::new(TokenType::NUMBER, "1.22".into(), 0),
            Token::new(TokenType::PLUS, "+".into(), 0),
            Token::new(TokenType::NUMBER, "5.22".into(), 0),
            Token::new(TokenType::STAR, "*".into(), 0),
            Token::new(TokenType::NUMBER, "21232.5347891".into(), 0),
        ];
        test_helper(source, expected);
    }
}
