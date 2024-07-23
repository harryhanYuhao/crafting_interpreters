use crate::scanner::*;
use crate::token::*;
use colored::*;

#[test]
fn test_scan_iteration() {
    let mut current = 0;
    let mut line = 1;
    let mut column = 1;
    let source = "!=".chars().collect::<Vec<char>>();
    let token = scan_iteration(&source, 0, &mut current, &mut line, &mut column, "stdin").unwrap();
    assert_eq!(token.as_ref().unwrap().token_type, TokenType::BANG_EQUAL);
    assert_eq!(token.as_ref().unwrap().lexeme, "!=");
    assert_eq!(token.as_ref().unwrap().line, 1);
}

#[test]
fn test_scan_tokens() {
    fn test_helper(source: &str) {
        let mut line = 1;
        let tokens = scan_tokens(source, &mut line, "stdin").unwrap();
        // DEBUG: BY PRINTING
        println!("{}: {}", "source".bright_blue().bold(), source);
        println!("{}", "Tokens:".cyan().bold());
        for i in tokens.iter() {
            println!("{:?}", i.lock().unwrap());
        }
    }

    let source = r#"! != "hello" 123 123.123"#;
    test_helper(source);

    let source = r#"1+2-3*4/5+1.22+5.22*21232.5347891"#;
    test_helper(source);

    let source = r#"var x = 10"#;
    test_helper(source);

    let source = 
r#"var x = 10; var y = 20; var z = 30; 
var a = 40;"#; 
    test_helper(source);
}
