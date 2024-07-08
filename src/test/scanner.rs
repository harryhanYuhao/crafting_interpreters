use crate::scanner::*;
use crate::token::*;
use colored::*;

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
        // DEBUG: BY PRINTING
        println!("{}: {}", "source".bright_blue().bold(), source);
        println!("{}", "Tokens:".cyan().bold());
        for i in tokens.iter() {
            println!("{:?}", i.lock().unwrap());
        }
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

    let source = r#"var x = 10"#;
    let expected = vec![
        Token::new(TokenType::VAR, "var".into(), 0),
        Token::new(TokenType::IDENTIFIER, "x".into(), 0),
        Token::new(TokenType::EQUAL, "=".into(), 0),
        Token::new(TokenType::NUMBER, "10".into(), 0),
    ];
    test_helper(source, expected);
}
