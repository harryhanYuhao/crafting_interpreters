use crate::interpreter::parser::{self, *};
use crate::interpreter::scanner::{self, *};
use crate::interpreter::token::*;
use crate::interpreter::AST_Node::{self, AST_Type, ExprType};
use colored::*;

#[test]
fn plus_minus_paren() {
    println!("{}", "Parser Tree:".cyan().bold());
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
    let res = parse(&mut parse_tree, "./test/plus_minus.lox");
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
        1,
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
        1,
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
        1,
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
        1,
    );
    assert_eq!(res, PatternMatchingRes::FailedAt(3));
}

#[test]
fn parser_match_ast_repetitive_pattern() {
    let string = "a = a = a =";
    let mut line_number = 1;
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
    let tokens: TokenArcVec = scanner::scan_tokens(string, &mut line_number, "stdin").unwrap();
    let mut tree = parser::ParseTreeUnfinshed::from(&tokens);
    // println!("{tree:?}");

    let res = tree.match_ast_repetitive_pattern(
        0,
        &vec![
            vec![AST_Type::Identifier],
            vec![AST_Type::Unparsed(TokenType::EQUAL)],
        ],
    );
    assert_eq!(res, (RepetitivePatternMatchingRes::MatchUntil(5), 0));

    let (res, _) = tree.match_ast_repetitive_pattern(
        0,
        &vec![
            vec![AST_Type::Unknown],
            vec![AST_Type::Unparsed(TokenType::EQUAL)],
        ],
    );
    assert_eq!(res, RepetitivePatternMatchingRes::Nomatch);

    let string = "a = a = a b";
    let mut line_number = 1;
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
    let tokens: TokenArcVec = scanner::scan_tokens(string, &mut line_number, "stdin").unwrap();
    let mut tree = parser::ParseTreeUnfinshed::from(&tokens);
    // println!("{tree:?}");

    let res = tree.match_ast_repetitive_pattern(
        0,
        &vec![
            vec![AST_Type::Identifier],
            vec![AST_Type::Unparsed(TokenType::EQUAL)],
            vec![AST_Type::Unparsed(TokenType::EQUAL)],
            vec![AST_Type::Unparsed(TokenType::EQUAL)],
            vec![AST_Type::Unparsed(TokenType::EQUAL)],
            vec![AST_Type::Unparsed(TokenType::EQUAL)],
            vec![AST_Type::Unparsed(TokenType::EQUAL)],
        ],
    );
    assert_eq!(res, (RepetitivePatternMatchingRes::MatchUntil(1), 0));
}

#[test]
fn delete_consect_stmt_sep() {
    let string = "a\n\n\n\nb";
    let mut line_number = 1;
    let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
    let tokens: TokenArcVec = scanner::scan_tokens(string, &mut line_number, "stdin").unwrap();

    let mut tree = parser::ParseTreeUnfinshed::from(&tokens);
    assert_eq!(delete_consec_stmt_sep_from_idx_inclusive(&mut tree, 0), 0);
    let mut tree = parser::ParseTreeUnfinshed::from(&tokens);
    assert_eq!(delete_consec_stmt_sep_from_idx_inclusive(&mut tree, 1), 4);
}
