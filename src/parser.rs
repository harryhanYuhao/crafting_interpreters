//! The purpose of the parser is to parse the token vector and return abstract syntax tree.
//! The token vectors is generated by the scanner

use crate::err_lox::*;
use crate::scanner;
use crate::token::{self, Token, TokenArcVec, TokenType};
use crate::AST_Node::*;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::iter::{Enumerate, FromIterator};
use std::ops::{Index, IndexMut, Sub};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref RVALUES: Vec<AST_Type> =
        [AST_Type::get_all_expr(), vec![AST_Type::Identifier]].concat();
    static ref COPULATIVE: Vec<AST_Type> = Vec::from([
        AST_Type::Unparsed(TokenType::PLUS),
        AST_Type::Unparsed(TokenType::PLUS_EQUAL),
        AST_Type::Unparsed(TokenType::MINUS),
        AST_Type::Unparsed(TokenType::MINUS_EQUAL),
        AST_Type::Unparsed(TokenType::STAR),
        AST_Type::Unparsed(TokenType::STAR_EQUAL),
        AST_Type::Unparsed(TokenType::SLASH),
        AST_Type::Unparsed(TokenType::SLASH_EQUAL),
        AST_Type::Unparsed(TokenType::EQUAL),
        AST_Type::Unparsed(TokenType::EQUAL_EQUAL),
        AST_Type::Unparsed(TokenType::BANG_EQUAL),
        AST_Type::Unparsed(TokenType::GREATER),
        AST_Type::Unparsed(TokenType::GREATER_EQUAL),
        AST_Type::Unparsed(TokenType::LESS),
        AST_Type::Unparsed(TokenType::LESS_EQUAL),
    ]);
}

// TODO: maybe it is unnecessary to have a separate struct for unfinished tree. Remove this struct.
pub struct ParseTreeUnfinshed {
    content: Vec<Arc<Mutex<AST_Node>>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternMatchingRes {
    Matched,
    Nomatch,
    FailedAt(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepetitivePatternMatchingRes {
    Nomatch,
    MatchUntil(usize),
}

impl ParseTreeUnfinshed {
    pub fn new() -> Self {
        ParseTreeUnfinshed {
            content: Vec::new(),
        }
    }

    pub fn push(&mut self, item: Arc<Mutex<AST_Node>>) {
        self.content.push(item);
    }

    pub fn extend(&mut self, ext: ParseTreeUnfinshed) {
        self.content.extend(ext.content)
    }

    pub fn remove(&mut self, index: usize) {
        self.content.remove(index);
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn slice(&mut self, start: usize, end: usize) -> Self {
        let content = self.content[start..end].to_vec();
        ParseTreeUnfinshed { content }
    }

    pub fn replace(&mut self, index: usize, node: Arc<Mutex<AST_Node>>) {
        self.content[index] = node;
    }

    // TODO: How to properly identify the tree is finished parsing
    pub fn get_finished_node(
        &self,
        source_file: &str,
    ) -> Result<Option<Arc<Mutex<AST_Node>>>, ErrorLox> {
        if self.content.len() > 1 {
            return Err(ErrorLox::from_arc_mutex_ast_node(
                self[0].clone(),
                "Internal Error: Parse Tree is unfinished when calling get_finished_node",
                source_file,
            ));
        }
        if self.content.len() == 1 {
            return Ok(Some(self.content[0].clone()));
        }
        Ok(None)
    }

    // TODO: How to properly identify the tree is finished parsing
    // Recall to also modify get_finished_node
    pub fn is_finished(&self) -> ParseState {
        if self.len() == 1 {
            return ParseState::Finished;
        }
        ParseState::Unfinished
    }

    /// check if the tree[index] is in patterns[0],
    /// tree[index + i] is in patterns[i].
    ///
    /// If tree[key_stmt_idx + index] does not belongs to patterns[key_stmt_idx], returns no_match,
    /// if tree[index] belongs to patterns[0] but one of the subsequent does not,
    /// returns FailAt(i), where i is the failed index.
    /// if everything matched, return matched.
    pub(crate) fn match_ast_pattern(
        &self,
        index: usize,
        patterns: &[Vec<AST_Type>],
        key_stmt_idx: usize,
    ) -> PatternMatchingRes {
        if key_stmt_idx + index >= self.len()
            || !AST_Node::arc_belongs_to_AST_type(
                self[index + key_stmt_idx].clone(),
                &patterns[key_stmt_idx],
            )
        {
            return PatternMatchingRes::Nomatch;
        }

        for (i, pattern) in patterns.iter().enumerate() {
            if i + index >= self.len()
                || !AST_Node::arc_belongs_to_AST_type(self[i + index].clone(), pattern)
            {
                if i == key_stmt_idx {
                    return PatternMatchingRes::Nomatch;
                } else {
                    return PatternMatchingRes::FailedAt(i);
                }
            }
        }

        PatternMatchingRes::Matched
    }

    /// Provided a tree and an index, check if the repetitive pattern matches.
    /// If not matched, return 0,
    /// if mathced, return th
    pub(crate) fn match_ast_repetitive_pattern(
        &self,
        index: usize,
        patterns: &[Vec<AST_Type>],
    ) -> RepetitivePatternMatchingRes {
        let mut matched_num = 0;
        let pattern_length = patterns.len();
        while AST_Node::arc_belongs_to_AST_type(
            self[index + matched_num].clone(),
            &patterns[matched_num % pattern_length],
        ) {
            matched_num += 1;
        }
        
        if matched_num == 0 {
            return RepetitivePatternMatchingRes::Nomatch;
        }
        RepetitivePatternMatchingRes::MatchUntil(matched_num - 1)
    }
}

impl FromIterator<Arc<Mutex<AST_Node>>> for ParseTreeUnfinshed {
    fn from_iter<I: IntoIterator<Item = Arc<Mutex<AST_Node>>>>(iter: I) -> Self {
        let mut res = ParseTreeUnfinshed::new();
        for i in iter {
            res.push(i)
        }
        res
    }
}

impl From<&TokenArcVec> for ParseTreeUnfinshed {
    fn from(s: &TokenArcVec) -> ParseTreeUnfinshed {
        s.iter()
            .map(|token| Arc::new(Mutex::new(AST_Node::from(Arc::clone(token)))))
            .collect()
    }
}

impl Index<usize> for ParseTreeUnfinshed {
    type Output = Arc<Mutex<AST_Node>>;

    fn index(&self, index: usize) -> &Arc<Mutex<AST_Node>> {
        &self.content[index]
    }
}

impl IndexMut<usize> for ParseTreeUnfinshed {
    fn index_mut(&mut self, index: usize) -> &mut Arc<Mutex<AST_Node>> {
        &mut self.content[index]
    }
}

#[derive(Debug)]
pub enum ParseState {
    Finished,
    Unfinished,
    Err(ErrorLox),
}

#[allow(unused_variables)]
impl fmt::Debug for ParseTreeUnfinshed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for (index, item) in self.content.iter().enumerate() {
            let tmp = item.clone();
            let tmp = tmp.lock().unwrap();
            res.push_str(&format!("{}", tmp));
            res.push_str("\n");
        }
        // remove the extra newline
        if self.content.len() > 0 {
            res.remove(res.len() - 1);
        }
        write!(f, "{}", res)
    }
}

/// Public API for parsing the tree
/// tree: parsed (may be unfinished)
/// tokens: more tokens to be parsed, which will be appended to the end of the tree
pub fn parse(tokens: &TokenArcVec, tree: &mut ParseTreeUnfinshed, source: &str) -> ParseState {
    let input_list = ParseTreeUnfinshed::from(tokens);
    println!("Input List:\n{:?}\n", input_list);
    tree.extend(input_list);

    real_parse(tree, source)
}

// parse the input strings into tokens, then feed the token into the unfinished parse tree, which
// is parsed
pub fn parse_from_string(
    string_input: &str,
    line_number: &mut usize,
    tree: &mut ParseTreeUnfinshed,
    source: &str,
) -> ParseState {
    let tokens: TokenArcVec = match scanner::scan_tokens(string_input, line_number, source) {
        Ok(ok) => ok,
        Err(e) => {
            return ParseState::Err(e);
        }
    };

    parse(&tokens, tree, source)
}

/// This is for subparse: not available for public api
/// The grand parse is reduced to several subparses, the order of which follows the  order of
/// precdence.
/// A finished subparse means there are no more avaiable tree to be made in that subparse, but the
/// whole parse may not be finished
/// A unfinished subparse means:
///     it needs more token from the next line to finish the subparse, in this case the parse is unfinished
/// Err means unrecoverable error. Terminated immediately
enum SubParseState {
    Unfinished,
    Finished,
    Err(ErrorLox),
}

macro_rules! error_handle_SubParseState {
    ($fun:expr) => {
        match $fun {
            SubParseState::Unfinished => return ParseState::Unfinished,
            SubParseState::Err(err) => return ParseState::Err(err),
            _ => {}
        }
    };
}

// this is the real parse. Define here for recursion
fn real_parse(tree: &mut ParseTreeUnfinshed, source: &str) -> ParseState {
    if tree.len() <= 1 {
        return ParseState::Finished;
    }

    error_handle_SubParseState!(parse_parenthesis(tree, source));
    error_handle_SubParseState!(parse_braces(tree, source));

    error_handle_SubParseState!(parse_prefix(
        tree,
        vec![AST_Type::Unparsed(TokenType::MINUS)],
        vec![
            AST_Type::Expr(ExprType::Normal),
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Identifier,
        ],
        [
            AST_Type::get_all_stmt(),
            vec![AST_Type::Unparsed(TokenType::STMT_SEP),],
            COPULATIVE.clone()
        ]
        .concat(),
        AST_Type::Expr(ExprType::Negated),
        source
    ));

    let plus_minus_ternery_valid_types =
        [AST_Type::get_all_expr(), vec![AST_Type::Identifier]].concat();

    // parse times, divide, and modular
    error_handle_SubParseState!(parse_ternary_left_assoc(
        tree,
        &RVALUES,
        &vec![TokenType::STAR, TokenType::SLASH, TokenType::PERCENT],
        &plus_minus_ternery_valid_types,
        AST_Type::Expr(ExprType::Normal),
        source
    ));
    // parse plus minus
    error_handle_SubParseState!(parse_ternary_left_assoc(
        tree,
        &plus_minus_ternery_valid_types,
        &vec![TokenType::PLUS, TokenType::MINUS],
        &plus_minus_ternery_valid_types,
        AST_Type::Expr(ExprType::Normal),
        source
    ));

    // parse >=, <=, >, <
    error_handle_SubParseState!(parse_ternary_left_assoc(
        tree,
        &plus_minus_ternery_valid_types,
        &vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL
        ],
        &plus_minus_ternery_valid_types,
        AST_Type::Expr(ExprType::Normal),
        source
    ));

    //parse ==, !=
    error_handle_SubParseState!(parse_ternary_left_assoc(
        tree,
        &plus_minus_ternery_valid_types,
        &vec![TokenType::EQUAL_EQUAL, TokenType::BANG_EQUAL],
        &plus_minus_ternery_valid_types,
        AST_Type::Expr(ExprType::Normal),
        source
    ));

    // println!("Before ASSIGNMENT:\n{tree:?}\nEND");
    // parsing assignment a = 2;
    error_handle_SubParseState!(parse_assignment_like(
        tree,
        vec![AST_Type::Unparsed(TokenType::EQUAL)],
        AST_Type::Stmt(StmtType::Assignment),
        source
    ));

    // parse a += 1
    error_handle_SubParseState!(parse_assignment_like(
        tree,
        vec![AST_Type::Unparsed(TokenType::PLUS_EQUAL),],
        AST_Type::Stmt(StmtType::PlusEqual),
        source
    ));

    // parse a -= 1
    error_handle_SubParseState!(parse_assignment_like(
        tree,
        vec![AST_Type::Unparsed(TokenType::MINUS_EQUAL)],
        AST_Type::Stmt(StmtType::MinusEqual),
        source
    ));

    // a *=1
    error_handle_SubParseState!(parse_assignment_like(
        tree,
        vec![AST_Type::Unparsed(TokenType::STAR_EQUAL)],
        AST_Type::Stmt(StmtType::StarEqual),
        source
    ));

    // a /= 1
    error_handle_SubParseState!(parse_assignment_like(
        tree,
        vec![AST_Type::Unparsed(TokenType::SLASH_EQUAL)],
        AST_Type::Stmt(StmtType::SlashEqual),
        source
    ));

    // parse var a = b;
    error_handle_SubParseState!(parse_prefix(
        tree,
        vec![AST_Type::Unparsed(TokenType::VAR)],
        vec![AST_Type::Stmt(StmtType::Assignment)],
        [
            AST_Type::get_all_stmt(),
            vec![AST_Type::Unparsed(TokenType::STMT_SEP)]
        ]
        .concat(),
        AST_Type::Stmt(StmtType::Declaration),
        source
    ));

    error_handle_SubParseState!(parse_comma(tree, &RVALUES, AST_Type::Tuple, source));

    error_handle_SubParseState!(parse_ternary_stmt_like_while(
        tree,
        &vec![AST_Type::Unparsed(TokenType::WHILE)],
        &vec![
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Normal)
        ],
        &vec![AST_Type::Stmt(StmtType::Braced)],
        AST_Type::Stmt(StmtType::While),
        source,
        "Expected expression after while",
        "Expected {stmt} after while",
    ));

    error_handle_SubParseState!(parse_if(tree, source));

    error_handle_SubParseState!(parse_stmt_sep(tree, source));
    error_handle_SubParseState!(parse_stmt_into_compound_stmt(tree));

    tree.is_finished()
}

/// trying to find the matching location for left ... right. left token and right token shall
/// behave like parenthesis. return a vector of tuple (start, end) holding the index of all of the
/// outermost matching delimiter
/// if there is no left or right exist at all, return vec of length 0
/// example:
/// tree = [], left = (, right = )
/// return Vec::new()
///
/// tree = (1), left = (, right = )
/// return [(0, 2)]
///
/// tree = 1 + (1 + 2), left = (, right = )
/// return [(2, 6)]
///
/// tree = 1 + (1 + 2) + (3+ (2+3)), left = (, right = )
/// return [(2, 6), (8, 16)]
pub(crate) fn get_delimiter_location(
    left: AST_Type,
    right: AST_Type,
    tree: &ParseTreeUnfinshed,
    source: &str,
) -> Result<Vec<(usize, usize)>, ErrorLox> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;
    let mut count = 0;
    let left = left.clone();
    for i in 0..tree.len() {
        match AST_Node::get_AST_Type_from_arc(tree[i].clone()) {
            y if y == left => {
                if count == 0 {
                    start = i;
                }
                count += 1;
            }
            y if y == right => {
                count -= 1;
                if count == 0 {
                    ret.push((start, i));
                }
                if count < 0 {
                    let e = ErrorLox::from_arc_mutex_ast_node(
                        tree[i].clone(),
                        "Extra right delimiter",
                        source,
                    );
                    return Err(e);
                }
            }
            _ => {}
        }
    }

    if count > 0 {
        let mut e = ErrorLox::from_arc_mutex_ast_node(
            tree[tree.len() - 1].clone(),
            &format!("Unpaired {:?}", left),
            source,
        );
        e.set_error_type(ErrorType::UnterminatedDelimiter);

        // println!("{e:?}");

        return Err(e);
    }

    Ok(ret)
}

// recursively parse parenthesis
fn parse_parenthesis(tree: &mut ParseTreeUnfinshed, source: &str) -> SubParseState {
    let locations = match get_delimiter_location(
        AST_Type::Unparsed(TokenType::LEFT_PAREN),
        AST_Type::Unparsed(TokenType::RIGHT_PAREN),
        &tree,
        source,
    ) {
        Ok(ok) => ok,
        // left and right paren must be at the same line
        Err(e) => return SubParseState::Err(e),
    };

    // recursive call;
    for (start, end) in locations.into_iter().rev() {
        let mut slice = tree.slice(start + 1, end);
        let sup_parse = real_parse(&mut slice, source);
        match sup_parse {
            ParseState::Err(e) => return SubParseState::Err(e),
            ParseState::Unfinished => {
                return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                    tree[start].clone(),
                    "Incomplete Inner Expr",
                    source,
                ));
            }
            ParseState::Finished => {
                let res = match slice.get_finished_node(source) {
                    Ok(ok) => ok,
                    Err(e) => return SubParseState::Err(e),
                };
                // the parse result may be none
                match res {
                    // in such case the parenethesis is just by itself
                    None => {
                        tree.remove(end);
                        AST_Node::set_arc_mutex_AST_Type(
                            tree[start].clone(),
                            AST_Type::Expr(ExprType::Paren),
                        );
                    }
                    Some(result) => {
                        for _ in (start + 1)..=(end) {
                            tree.remove(start + 1);
                        }
                        // tree.replace(start, result);
                        AST_Node::set_arc_mutex_AST_Type(
                            tree[start].clone(),
                            AST_Type::Expr(ExprType::Paren),
                        );
                        AST_Node::arc_mutex_append_child(tree[start].clone(), result);
                    }
                }
            }
        }
    }
    SubParseState::Finished
}

fn parse_braces(tree: &mut ParseTreeUnfinshed, source: &str) -> SubParseState {
    let locations = match get_delimiter_location(
        AST_Type::Unparsed(TokenType::LEFT_BRACE),
        AST_Type::Unparsed(TokenType::RIGHT_BRACE),
        &tree,
        source,
    ) {
        Ok(ok) => {
            println!("{ok:?}");
            ok
        }
        Err(e) => {
            println!("error: {e:?}");
            match e.get_error_type() {
                ErrorType::UnterminatedDelimiter => {
                    return SubParseState::Unfinished;
                }
                _ => {
                    return SubParseState::Err(e);
                }
            }
        }
    };

    // recursive call;
    for (start, end) in locations.into_iter().rev() {
        let mut slice = tree.slice(start + 1, end);
        println!("SLICES: \n{:?}\nEND", slice);

        let sup_parse = real_parse(&mut slice, source);
        println!("SLICES after parse: \n{:?}\nEND", slice);
        match sup_parse {
            ParseState::Err(e) => return SubParseState::Err(e),
            ParseState::Unfinished => {
                return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                    tree[start].clone(),
                    "Incomplete Inner Expr",
                    source,
                ));
            }
            ParseState::Finished => {
                let res = match slice.get_finished_node(source) {
                    Ok(ok) => ok,
                    Err(e) => return SubParseState::Err(e),
                };
                // the parse result may be none
                // If there is result, the result is presented by one compound
                // stmt, which is redundant
                match res {
                    // in such case the parenethesis is just by itself
                    None => {
                        tree.remove(end);
                        AST_Node::set_arc_mutex_AST_Type(
                            tree[start].clone(),
                            AST_Type::Stmt(StmtType::Braced),
                        );
                    }
                    Some(result) => {
                        for _ in (start + 1)..=(end) {
                            tree.remove(start + 1);
                        }
                        // tree.replace(start, result);
                        AST_Node::set_arc_mutex_AST_Type(
                            tree[start].clone(),
                            AST_Type::Stmt(StmtType::Braced),
                        );
                        AST_Node::arc_mutex_append_children(
                            tree[start].clone(),
                            &AST_Node::arc_mutex_get_children(result.clone()),
                        );
                    }
                }
            }
        }
    }
    SubParseState::Finished
}

// TODO: REFACTOR WITH AST_MATCH

/// This function constructs the ternary left associtive operators into tree, whose grammer is
/// similar to +, -, *, /
///
/// If the operator is found, but left_ast_types or right_ast_types are not found,  
fn parse_ternary_left_assoc(
    tree: &mut ParseTreeUnfinshed,
    left_ast_types: &[AST_Type],
    operator_token_types: &[TokenType],
    right_ast_types: &[AST_Type],
    result_type: AST_Type,
    source: &str,
) -> SubParseState {
    // if the operator appears at the beginning or at the end, return error.
    if AST_Node::arc_belongs_to_Token_type(tree[0].clone(), operator_token_types) {
        return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
            tree[0].clone(),
            &format!("{:?} found in the beginning", operator_token_types),
            source,
        ));
    }
    if AST_Node::arc_belongs_to_Token_type(tree[tree.len() - 1].clone(), operator_token_types) {
        return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
            tree[tree.len() - 1].clone(),
            &format!("{:?} found in the end", operator_token_types),
            source,
        ));
    }

    // Start of parsing

    let mut length = tree.len();
    let mut i = 0;

    // ignore the last two tokens
    while i + 2 < length {
        if !AST_Node::arc_belongs_to_Token_type(tree[i + 1].clone(), operator_token_types) {
            i += 1;
            continue;
        }
        // match the type of the first token
        if !AST_Node::arc_belongs_to_AST_type(tree[i].clone(), left_ast_types) {
            i += 1;
            continue;
        }
        // check the third toklen
        if !AST_Node::arc_belongs_to_AST_type(tree[i + 2].clone(), right_ast_types) {
            let e = ErrorLox::from_arc_mutex_ast_node(
                tree[i + 2].clone(),
                &format!("expected {:?}", right_ast_types),
                source,
            );
            return SubParseState::Err(e);
        }
        // Construct the tree
        {
            let mut root = tree[i + 1].lock().unwrap();
            root.set_AST_Type(result_type.clone());
            root.append_child(tree[i].clone());
            root.append_child(tree[i + 2].clone());
        }
        // remove the first expr,
        // note the length of the array decreases by one
        tree.remove(i);
        // remove the second expr
        tree.remove(i + 1);
        length -= 2;
        // skipping i += 1; the new node needs to be parsed again
    }
    SubParseState::Finished
}

/// parse statements like expr; into stmt(normal) -> expr
fn parse_post_single(
    tree: &mut ParseTreeUnfinshed,
    ast_type: &[AST_Type],
    operator_token_type: &[TokenType],
    result_type: AST_Type,
) -> SubParseState {
    let mut length = tree.len();
    let mut i = 0;

    while i + 1 < length {
        if !AST_Node::arc_belongs_to_AST_type(tree[i].clone(), ast_type) {
            i += 1;
            continue;
        }
        if !AST_Node::arc_belongs_to_Token_type(tree[i + 1].clone(), operator_token_type) {
            i += 1;
            continue;
        }
        let node = tree[i + 1].clone();
        let mut node = node.lock().unwrap();
        node.set_AST_Type(result_type.clone());
        node.append_child(tree[i].clone());
        tree.remove(i);
        length -= 1;
        i += 1;
    }
    SubParseState::Finished
}

/// parse var ast_assignment into tree
// fn parse_var_old(
//     tree: &mut ParseTreeUnfinshed,
//     operator_token_type: &[TokenType],
//     ast_type: &[AST_Type],
//     result_type: AST_Type,
//     source: &str,
// ) -> SubParseState {
//     let mut length = tree.len();
//     let mut i = 0;
//
//     while i + 1 < length {
//         if !AST_Node::arc_belongs_to_Token_type(tree[i].clone(), operator_token_type) {
//             i += 1;
//             continue;
//         }
//         if !AST_Node::arc_belongs_to_AST_type(tree[i + 1].clone(), ast_type) {
//             return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
//                 tree[i + 1].clone(),
//                 "Expected Assignment Stmt",
//                 source,
//             ));
//         }
//
//         let node = tree[i + 1].clone();
//         let mut node = node.lock().unwrap();
//         node.set_AST_Type(result_type.clone());
//         tree.remove(i);
//         length -= 1;
//         i += 1;
//     }
//     SubParseState::Finished
// }

/// The final, finished parse tree shall consist of a single root of type Stmt(Compound). All of
/// the substatment shall be children of this node.
/// This function arrange vector of statement into one node.
/// It creates an empty compound node at first and scans the tree
/// If found a lone statement, the lone statement is appended into the compound node. If found a
/// compound statement, two compound are merge. The compound statement was then inserted into the
/// tree properly
/// If expressions are found, they are left alone. This is important, as the result of parsing
/// could be an expression and not a statement
fn parse_stmt_into_compound_stmt(tree: &mut ParseTreeUnfinshed) -> SubParseState {
    let mut length = tree.len();
    let mut i = 0;

    let mut compound: Arc<Mutex<AST_Node>> =
        AST_Node::new(AST_Type::Stmt(StmtType::Compound), Token::dummy()).into();

    while i < length {
        if AST_Node::is_arc_mutex_stmt(tree[i].clone()) {
            let has_child: bool = AST_Node::arc_mutex_has_children(compound.clone());
            if AST_Node::is_arc_mutex_compound_stmt(tree[i].clone()) {
                let node = tree[i].clone();
                let node = node.lock().unwrap();
                for i in node.get_children() {
                    AST_Node::arc_mutex_append_child(compound.clone(), i.clone());
                }
            } else {
                AST_Node::arc_mutex_append_child(compound.clone(), tree[i].clone());
            }
            if has_child {
                tree.remove(i);
                length -= 1;
            } else {
                tree.replace(i, compound.clone());
                i += 1;
            }
        } else {
            // in such case the node is expr
            i += 1;
            if AST_Node::arc_mutex_has_children(compound.clone()) {
                compound = AST_Node::new(AST_Type::Stmt(StmtType::Compound), Token::dummy()).into();
            }
        }
    }

    SubParseState::Finished
}

/// parse statements like
/// ```a = 1+2``` (identifier, equal, expr, stmt_sep)
fn parse_assignment_like(
    tree: &mut ParseTreeUnfinshed,
    key_ast_type: Vec<AST_Type>,
    result_type: AST_Type,
    source: &str,
) -> SubParseState {
    let mut i = 0;
    let mut length = tree.len();
    let expected = vec![
        vec![AST_Type::Identifier],
        // vec![AST_Type::Unparsed(TokenType::EQUAL)],
        key_ast_type,
        RVALUES.clone(),
        vec![AST_Type::Unparsed(TokenType::STMT_SEP)],
    ];
    // println!("Assignment like: \n{:?}", tree);
    while i < length {
        let res = tree.match_ast_pattern(i, &expected, 1);

        match res {
            PatternMatchingRes::Matched => {
                {
                    let mut root = tree[i + 1].lock().unwrap();
                    root.set_AST_Type(result_type.clone());
                    root.append_child(tree[i].clone());
                    root.append_child(tree[i + 2].clone());
                }
                // remove the first expr,
                // note the length of the array decreases by one
                tree.remove(i);
                // remove the second expr
                tree.remove(i + 1);
                tree.remove(i + 1);
                length -= 3;
            }
            PatternMatchingRes::FailedAt(num) => {
                return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                    tree[i + num].clone(),
                    &format!("Expected {:?}", expected[num]),
                    source,
                ))
            }
            _ => {}
        }
        i += 1;
    }

    SubParseState::Finished
}

fn parse_prefix(
    tree: &mut ParseTreeUnfinshed,
    prefix_types: Vec<AST_Type>,
    sequential_type: Vec<AST_Type>,
    valid_after_ast_type: Vec<AST_Type>,
    result_type: AST_Type,
    source: &str,
) -> SubParseState {
    let mut i = 0;
    let mut length = tree.len();
    let expected = vec![prefix_types, sequential_type];

    while i < length {
        // Only parse if it is prefix
        if i > 0 {
            if !AST_Node::arc_belongs_to_AST_type(tree[i - 1].clone(), &valid_after_ast_type) {
                i += 1;
                continue;
            }
        }
        let res = tree.match_ast_pattern(i, &expected, 0);

        match res {
            PatternMatchingRes::Nomatch => {}
            PatternMatchingRes::FailedAt(num) => {
                return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                    tree[i + num].clone(),
                    &format!("Expected {:?}", expected[num]),
                    source,
                ))
            }
            PatternMatchingRes::Matched => {
                let node = tree[i + 1].clone();
                let mut node = node.lock().unwrap();
                node.set_AST_Type(result_type.clone());
                tree.remove(i);
                length -= 1;
            }
        }
        i += 1;
    }

    SubParseState::Finished
}

fn parse_stmt_sep(tree: &mut ParseTreeUnfinshed, source: &str) -> SubParseState {
    let mut i = 0;
    let mut length = tree.len();

    while i < length {
        if AST_Node::get_AST_Type_from_arc(tree[i].clone())
            == AST_Type::Unparsed(TokenType::STMT_SEP)
        {
            if i == 0 {
                tree.remove(i);
                length -= 1;
                continue;
            }
            if AST_Node::is_arc_mutex_stmt(tree[i - 1].clone()) {
                tree.remove(i);
                length -= 1;
                continue;
            }
            // TODO: IS THIS HANDLING WHAT WE EXPECTED?
            let node = tree[i].clone();
            let mut node = node.lock().unwrap();
            node.set_AST_Type(AST_Type::Stmt(StmtType::Normal));
            node.append_child(tree[i - 1].clone());
            tree.remove(i - 1);
            length -= 1;
            continue;
            // return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
            //     tree[i - 1].clone(),
            //     "Unexpected Node, likely an internal error.",
            //     source,
            // ));
        }
        i += 1;
    }
    SubParseState::Finished
}

/// Parse syntax like `while expr {stmt}`
/// Error Handling:
/// Err:: expected expressiong if
/// while \n expr {stmt} or while {stmt} or while (by itself)
/// Err:: expected {stmt} if
/// while expr or while expr stmt
fn parse_ternary_stmt_like_while(
    tree: &mut ParseTreeUnfinshed,
    operator_token_types: &[AST_Type],
    left_ast_types: &[AST_Type],
    right_ast_types: &[AST_Type],
    result_type: AST_Type,
    source: &str,
    error_1: &str,
    error_2: &str,
) -> SubParseState {
    let mut length = tree.len();
    let mut i = 0;
    // ignore the last two tokens
    while i + 2 < length {
        // match the type of the first token
        if !AST_Node::arc_belongs_to_AST_type(tree[i].clone(), operator_token_types) {
            i += 1;
            continue;
        }
        if !AST_Node::arc_belongs_to_AST_type(tree[i + 1].clone(), left_ast_types) {
            return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                tree[i].clone(),
                error_1,
                source,
            ));
        }
        // check the third toklen
        if !AST_Node::arc_belongs_to_AST_type(tree[i + 2].clone(), right_ast_types) {
            return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                tree[i].clone(),
                error_2,
                source,
            ));
        }
        // Construct the tree
        {
            let mut root = tree[i].lock().unwrap();
            root.set_AST_Type(result_type.clone());
            root.append_child(tree[i + 1].clone());
            root.append_child(tree[i + 2].clone());
        }
        // remove the extra nodes,
        // note the length of the array decreases by two
        tree.remove(i + 1);
        tree.remove(i + 1);
        length -= 2;
    }
    SubParseState::Finished
}

fn parse_comma(
    tree: &mut ParseTreeUnfinshed,
    expected_types: &[AST_Type],
    result_type: AST_Type,
    source: &str,
) -> SubParseState {
    let mut i = 0;
    let mut length = tree.len();

    while i < length {
        if AST_Node::get_AST_Type_from_arc(tree[i].clone()) != AST_Type::Unparsed(TokenType::COMMA)
        {
            i += 1;
            continue;
        }

        // now tree[i] is comma
        let mut comma_count = 0;

        while AST_Node::get_AST_Type_from_arc(tree[i + comma_count * 2].clone())
            == AST_Type::Unparsed(TokenType::COMMA)
        {
            comma_count += 1;
            if i + (comma_count - 1) * 2 == 0
                || !AST_Node::arc_belongs_to_AST_type(
                    tree[i + (comma_count - 1) * 2 - 1].clone(),
                    expected_types,
                )
            {
                let error_idx = match i + (comma_count - 1) * 2 {
                    tmp if tmp == 0 => 0,
                    tmp => tmp - 1,
                };
                return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                    tree[error_idx].clone(),
                    &format!("Expected {expected_types:?}"),
                    source,
                ));
            }
            if i + (comma_count - 1) * 2 == length - 1
                || !AST_Node::arc_belongs_to_AST_type(
                    tree[i + (comma_count - 1) * 2 + 1].clone(),
                    expected_types,
                )
            {
                let error_idx = match i + (comma_count - 1) * 2 {
                    tmp if tmp == length - 1 => 0,
                    tmp => tmp + 1,
                };
                return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                    tree[error_idx].clone(),
                    &format!("Expected {expected_types:?}"),
                    source,
                ));
            }
        }

        for j in 0..=comma_count {
            AST_Node::arc_mutex_append_child(tree[i].clone(), tree[i - 1 + j * 2].clone())
        }
        AST_Node::set_arc_mutex_AST_Type(tree[i].clone(), result_type.clone());

        for j in ((i + 1)..=(i + comma_count * 2 - 1)).rev() {
            tree.remove(j);
        }
        tree.remove(i - 1);

        length -= 2 * comma_count;
        i += 1;
    }

    SubParseState::Finished
}

fn parse_if(tree: &mut ParseTreeUnfinshed, source: &str) -> SubParseState {
    let mut i = 0;
    let mut length = tree.len();

    while i < length {
        if AST_Node::get_AST_Type_from_arc(tree[i].clone()) != AST_Type::Unparsed(TokenType::IF) {
            i += 1;
            continue;
        }

        if i + 1 == length - 1 || !AST_Node::arc_belongs_to_AST_type(tree[i + 1].clone(), &RVALUES)
        {
            let error_idx = match i + 1 {
                tmp if tmp == length - 1 => length - 1,
                tmp => tmp + 1,
            };
            return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                tree[error_idx].clone(),
                &format!("Expected rvalues",),
                source,
            ));
        }

        if i + 2 == length - 1
            || !AST_Node::arc_belongs_to_AST_type(
                tree[i + 2].clone(),
                &vec![AST_Type::Stmt(StmtType::Braced)],
            )
        {
            let error_idx = match i + 2 {
                tmp if tmp == length - 1 => length - 1,
                tmp => tmp + 2,
            };
            return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                tree[error_idx].clone(),
                &format!("Expected Braced Statment",),
                source,
            ));
        }

        // The first step of parsing: disregarding else and else if for now
        {
            let mut root = tree[i].lock().unwrap();
            root.set_AST_Type(AST_Type::Stmt(StmtType::If));
            root.append_child(tree[i + 1].clone());
            root.append_child(tree[i + 2].clone());
        }
        // remove the extra nodes,
        // note the length of the array decreases by two
        tree.remove(i + 1);
        tree.remove(i + 1);
        length -= 2;
        i += 1;

        // handling else and else if
        if i < length
            && AST_Node::get_AST_Type_from_arc(tree[i].clone())
                != AST_Type::Unparsed(TokenType::ELSE)
        {
            i += 1;
            continue;
        }

        // tree[i] is else
        if i + 1 == length
            || !AST_Node::arc_belongs_to_AST_type(
                tree[i + 1].clone(),
                &vec![
                    AST_Type::Stmt(StmtType::Braced),
                    AST_Type::Unparsed(TokenType::IF),
                ],
            )
        {
            return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                tree[i + 1].clone(),
                &format!("Expected Braced Statment or if after else",),
                source,
            ));
        }

        // else {}

        if AST_Node::get_AST_Type_from_arc(tree[i + 1].clone()) == AST_Type::Stmt(StmtType::Braced)
        {
            AST_Node::arc_mutex_append_child(tree[i - 1].clone(), tree[i + 1].clone());
            tree.remove(i + 1);
            tree.remove(i);
            length -= 2;
            i += 2;
            continue;
        }

        // in this case tree[i+1] is if. Check if braced stmt follows

        if i + 2 == length
            || !AST_Node::arc_belongs_to_AST_type(
                tree[i + 2].clone(),
                &vec![AST_Type::Stmt(StmtType::Braced)],
            )
        {
            return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                tree[i + 1].clone(),
                &format!("Expected Braced Statment after else",),
                source,
            ));
        }
    }
    SubParseState::Finished
}
