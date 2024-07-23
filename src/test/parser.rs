use crate::parser::{self, *};
use crate::scanner::{self, *};
use crate::token::*;
use colored::*;

#[test]
fn plus_minus_paren() {
    let input = "1-2";
    // println!("{}: {}", "input".bright_blue().bold(), input);
    println!("{}", "Parser Tree:".cyan().bold());
    let mut line = 0;
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
    let res = parse_from_string(input, &mut line, &mut parse_tree, "sdtin");
    match res {
        ParseState::Err(e) => {
            panic!("{}: {}", "Error".red().bold(), format!("{:?}", e).bold());
        }
        _ => {}
    }
    println!("{:?}", parse_tree);
}

#[test]
fn delimiter_location() {
    let mut line = 0;
    let input = "{1 - 2}";
    let tokens: TokenArcVec = scanner::scan_tokens(input, &mut line, "stdin").unwrap();
    let parse_tree = ParseTreeUnfinshed::from(&tokens);
    println!("{:?}", parse_tree);
    println!(
        "{:?}",
        get_delimiter_location(
            TokenType::LEFT_PAREN,
            TokenType::RIGHT_PAREN,
            &parse_tree,
            "stdin",
        )
    );
}

#[test]
fn parser_match_ast_pattern() {
    let string = "a = 2";
    let mut line_number = 1;
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
    let tokens: TokenArcVec = scanner::scan_tokens(string, &mut line_number, "stdin").unwrap();
    
    let res = parser::parse(&tokens, &mut parse_tree, "stdin");
    println!("{:?}", parse_tree);
}
