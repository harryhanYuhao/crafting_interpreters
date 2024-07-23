use crate::parser::{self, *};
use crate::scanner::{self, *};
use crate::token::*;
use crate::AST_Node::{self, ExprType};
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
    let tree = parser::ParseTreeUnfinshed::from(&tokens);
    let res = tree.match_ast_pattern(
        0,
        &vec![
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Unparsed(TokenType::EQUAL)],
            vec![
                AST_Node::AST_Type::Expr(ExprType::Paren),
                AST_Node::AST_Type::Expr(ExprType::Normal),
            ],
        ],
    );
    assert_eq!(res, PatternMatchingRes::Matched);

    let res = tree.match_ast_pattern(
        0,
        &vec![
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Unparsed(TokenType::EQUAL)],
            vec![
                AST_Node::AST_Type::Expr(ExprType::Paren),
                AST_Node::AST_Type::Expr(ExprType::Normal),
            ],
        ],
    );
    assert_eq!(res, PatternMatchingRes::FailedAt(1));

    let res = tree.match_ast_pattern(
        0,
        &vec![
            vec![AST_Node::AST_Type::Unparsed(TokenType::EQUAL)],
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Identifier],
            vec![
                AST_Node::AST_Type::Expr(ExprType::Paren),
                AST_Node::AST_Type::Expr(ExprType::Normal),
            ],
        ],
    );
    assert_eq!(res, PatternMatchingRes::Nomatch);

    let res = tree.match_ast_pattern(
        0,
        &vec![
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Identifier],
            vec![
                AST_Node::AST_Type::Expr(ExprType::Paren),
                AST_Node::AST_Type::Expr(ExprType::Normal),
            ],
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Identifier],
        ],
    );
    assert_eq!(res, PatternMatchingRes::FailedAt(1));

    let res = tree.match_ast_pattern(
        10,
        &vec![
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Identifier],
            vec![
                AST_Node::AST_Type::Expr(ExprType::Paren),
                AST_Node::AST_Type::Expr(ExprType::Normal),
            ],
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Identifier],
        ],
    );
    assert_eq!(res, PatternMatchingRes::Nomatch);
}
