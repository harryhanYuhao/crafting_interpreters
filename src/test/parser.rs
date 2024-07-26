use crate::parser::{self, *};
use crate::scanner::{self, *};
use crate::token::*;
use crate::AST_Node::{self, ExprType, AST_Type};
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
            AST_Type::Unparsed(TokenType::LEFT_PAREN),
            AST_Type::Unparsed(TokenType::RIGHT_PAREN),
            &parse_tree,
            "stdin",
        )
    );
}

#[test]
fn parser_match_ast_pattern() {
    let string = "a = 2;";
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
            vec![AST_Node::AST_Type::Unparsed(TokenType::STMT_SEP)],
        ],
        1
    );
    assert_eq!(res, PatternMatchingRes::Matched);

    let res = tree.match_ast_pattern(
        0,
        &vec![
            vec![AST_Node::AST_Type::Unknown],
            vec![AST_Node::AST_Type::Unparsed(TokenType::EQUAL)],
            vec![
                AST_Node::AST_Type::Expr(ExprType::Paren),
                AST_Node::AST_Type::Expr(ExprType::Normal),
            ],
            vec![AST_Node::AST_Type::Unparsed(TokenType::STMT_SEP)],
        ],
        1
    );
    assert_eq!(res, PatternMatchingRes::FailedAt(0));

    let res = tree.match_ast_pattern(
        0,
        &vec![
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Unparsed(TokenType::EOF)],
            vec![
                AST_Node::AST_Type::Expr(ExprType::Paren),
                AST_Node::AST_Type::Expr(ExprType::Normal),
            ],
            vec![AST_Node::AST_Type::Unparsed(TokenType::STMT_SEP)],
        ],
        1
    );
    assert_eq!(res, PatternMatchingRes::Nomatch);

    let res = tree.match_ast_pattern(
        0,
        &vec![
            vec![AST_Node::AST_Type::Identifier],
            vec![AST_Node::AST_Type::Unparsed(TokenType::EQUAL)],
            vec![
                AST_Node::AST_Type::Expr(ExprType::Paren),
                AST_Node::AST_Type::Expr(ExprType::Normal),
            ],
            vec![AST_Node::AST_Type::Unparsed(TokenType::EOF)],
        ],
        1
    );
    assert_eq!(res, PatternMatchingRes::FailedAt(3));
}

#[test]
fn parser_match_ast_repetitive_pattern(){
    let string = "a = a = a =";
    let mut line_number = 1;
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
    let tokens: TokenArcVec = scanner::scan_tokens(string, &mut line_number, "stdin").unwrap();
    let tree = parser::ParseTreeUnfinshed::from(&tokens);
    // println!("{tree:?}");
    
    let res = tree.match_ast_repetitive_pattern(0, &vec![
        vec![AST_Type::Identifier],
        vec![AST_Type::Unparsed(TokenType::EQUAL)],
    ]);
    assert_eq!(res, RepetitivePatternMatchingRes::MatchUntil(5));

    let res = tree.match_ast_repetitive_pattern(0, &vec![
        vec![AST_Type::Unknown],
        vec![AST_Type::Unparsed(TokenType::EQUAL)],
    ]);
    assert_eq!(res, RepetitivePatternMatchingRes::Nomatch);
}
