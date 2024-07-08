use crate::parser::*;
use crate::scanner;
use crate::token::*;
use colored::*;

#[test]
fn plus_minus_paren() {
    let input = "x + 1 + 2 * (5 + 3 - 2 * (4 / (7 + 2)))";
    println!("{}: {}", "input".bright_blue().bold(), input);
    println!("{}", "Parser Tree:".cyan().bold());
    let mut line = 0;
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
    let tokens: TokenArcVec = scanner::scan_tokens(input, &mut line).unwrap();
    let res = parse(&tokens, &mut parse_tree);
    match res {
        ParseState::Err(e) => {
            panic!("{}: {}", "Error".red().bold(), format!("{:?}", e).bold());
        }
        _ => {}
    }
    println!("{:?}", parse_tree);
}
