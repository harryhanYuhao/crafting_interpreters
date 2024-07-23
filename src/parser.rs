//! The purpose of the parser is to parse the token vector and return abstract syntax tree.
//! The token vectors is generated by the scanner

use crate::err_lox::*;
use crate::token::{self, Token, TokenArcVec, TokenType};
use crate::AST_Node::*;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::iter::{Enumerate, FromIterator};
use std::ops::{Index, IndexMut, Sub};
use std::sync::{Arc, Mutex};

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
    // TODO: Proper error handling
    pub fn get_finished_node(&self) -> Option<Arc<Mutex<AST_Node>>> {
        if self.content.len() > 1 {
            panic!("Internal Error: Tree is in unfinished state!");
        }
        if self.content.len() == 1 {
            return Some(self.content[0].clone());
        }
        None
    }

    // TODO: How to properly identify the tree is finished parsing
    // Recall to also modify get_finished_node
    pub fn is_finished(&self) -> ParseState {
        if self.len() == 1 {
            return ParseState::Finished;
        }
        ParseState::Unfinished
    }

    pub(crate) fn match_ast_pattern(
        &self,
        index: usize,
        patterns: Vec<AST_Type>,
    ) -> PatternMatchingRes {
        let pattern_len = patterns.len();
        if index + pattern_len > self.len() {
            return PatternMatchingRes::Nomatch;
        }
        for (i, pattern) in patterns.iter().enumerate() {
            if AST_Node::get_AST_Type_from_arc(self[index + i].clone()) != pattern.clone() {
                if i == 0 {
                    return PatternMatchingRes::Nomatch;
                } else {
                    return PatternMatchingRes::FailedAt(i);
                }
            }
        }

        PatternMatchingRes::Matched
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
pub fn parse(tokens: &TokenArcVec, tree: &mut ParseTreeUnfinshed, source: &str) -> ParseState {
    let input_list = ParseTreeUnfinshed::from(tokens);
    tree.extend(input_list);

    return real_parse(tree, source);
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

    // parse times, divide, and modular
    error_handle_SubParseState!(parse_ternary_left_assoc(
        tree,
        &vec![
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Normal),
            AST_Type::PotentialStmt,
            AST_Type::Identifier
        ],
        &vec![TokenType::STAR, TokenType::SLASH, TokenType::PERCENT],
        &vec![
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Normal),
            AST_Type::PotentialStmt,
            AST_Type::Identifier
        ],
        AST_Type::Expr(ExprType::Normal),
    ));
    // parse plus minus
    error_handle_SubParseState!(parse_ternary_left_assoc(
        tree,
        &vec![
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Normal),
            AST_Type::PotentialStmt,
            AST_Type::Identifier
        ],
        &vec![TokenType::PLUS, TokenType::MINUS],
        &vec![
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Normal),
            AST_Type::PotentialStmt,
            AST_Type::Identifier
        ],
        AST_Type::Expr(ExprType::Normal),
    ));

    // parse assignment
    error_handle_SubParseState!(parse_ternary_left_assoc(
        tree,
        &vec![AST_Type::Identifier],
        &vec![TokenType::EQUAL],
        &vec![
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Normal),
            AST_Type::Identifier
        ],
        AST_Type::Stmt(StmtType::Assignment),
    ));

    error_handle_SubParseState!(parse_var(
        tree,
        &vec![TokenType::VAR],
        &vec![AST_Type::Stmt(StmtType::Assignment)],
        AST_Type::Stmt(StmtType::Declaration),
        source,
    ));

    // parse statements like expr; into stmt(normal) -> expr
    error_handle_SubParseState!(parse_post_single(
        tree,
        &vec![
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Normal)
        ],
        &vec![TokenType::STMT_SEP],
        AST_Type::Stmt(StmtType::Normal),
    ));

    error_handle_SubParseState!(parse_ternary_token_asttype_asttype(
        tree,
        &vec![TokenType::WHILE],
        &vec![
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Normal)
        ],
        &vec![AST_Type::Stmt(StmtType::Braced)],
        AST_Type::Stmt(StmtType::While),
        source,
        "Expected expression after if",
        "Expected {stmt} after if",
    ));

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
    left: TokenType,
    right: TokenType,
    tree: &ParseTreeUnfinshed,
    source: &str,
) -> Result<Vec<(usize, usize)>, ErrorLox> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    let mut start = 0;
    let mut count = 0;
    let left = left.clone();
    for i in 0..tree.len() {
        match AST_Node::get_token_type_from_arc(tree[i].clone()) {
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
            "Unpaired left delimiter",
            source,
        );
        e.set_error_type(ErrorType::UnterminatedDelimiter);
        return Err(e);
    }

    Ok(ret)
}

// recursively parse parenthesis
fn parse_parenthesis(tree: &mut ParseTreeUnfinshed, source: &str) -> SubParseState {
    let locations = match get_delimiter_location(
        TokenType::LEFT_PAREN,
        TokenType::RIGHT_PAREN,
        &tree,
        source,
    ) {
        Ok(ok) => ok,
        Err(e) => return SubParseState::Err(e),
    };

    for (start, end) in locations.into_iter().rev() {
        // the work begin
        // recursive call;
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
                let res = slice.get_finished_node();
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
        TokenType::LEFT_BRACE,
        TokenType::RIGHT_BRACE,
        &tree,
        source,
    ) {
        Ok(ok) => ok,
        Err(e) => match e.get_error_type() {
            ErrorType::UnterminatedDelimiter => {
                return SubParseState::Unfinished;
            }
            _ => {
                return SubParseState::Err(e);
            }
        },
    };

    for (start, end) in locations.into_iter().rev() {
        // the work begin
        // recursive call;
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
                let res = slice.get_finished_node();
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
/// This function constructs the ternary left associtive operators into tree, whose grammer is
/// similar to +, -, *, /
/// It will set the resulting AST_Type as expr(normal)
/// TODO:
/// It shall return error of the operator starts in the beginning of the line or at the end of the
/// line. But recall we are parsing the whole assembled tree
fn parse_ternary_left_assoc(
    tree: &mut ParseTreeUnfinshed,
    left_ast_types: &[AST_Type],
    operator_token_types: &[TokenType],
    right_ast_types: &[AST_Type],
    result_type: AST_Type,
) -> SubParseState {
    // if the operator appears at the beginning or at the end, return error.

    let mut length = tree.len();
    let mut i = 0;

    // ignore the last two tokens
    while i + 2 < length {
        // match the type of the first token
        if !AST_Node::arc_belongs_to_AST_type(tree[i].clone(), left_ast_types) {
            i += 1;
            continue;
        }
        // check the second token
        if !AST_Node::arc_belongs_to_Token_type(tree[i + 1].clone(), operator_token_types) {
            i += 1;
            continue;
        }

        // check the third toklen
        if !AST_Node::arc_belongs_to_AST_type(tree[i + 2].clone(), right_ast_types) {
            return SubParseState::Unfinished;
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
fn parse_var(
    tree: &mut ParseTreeUnfinshed,
    operator_token_type: &[TokenType],
    ast_type: &[AST_Type],
    result_type: AST_Type,
    source: &str,
) -> SubParseState {
    let mut length = tree.len();
    let mut i = 0;

    while i + 1 < length {
        if !AST_Node::arc_belongs_to_Token_type(tree[i].clone(), operator_token_type) {
            i += 1;
            continue;
        }
        if !AST_Node::arc_belongs_to_AST_type(tree[i + 1].clone(), ast_type) {
            return SubParseState::Err(ErrorLox::from_arc_mutex_ast_node(
                tree[i + 1].clone(),
                "Expected Assignment Stmt",
                source,
            ));
        }

        let node = tree[i + 1].clone();
        let mut node = node.lock().unwrap();
        node.set_AST_Type(result_type.clone());
        tree.remove(i);
        length -= 1;
        i += 1;
    }
    SubParseState::Finished
}

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

/// Parse syntax like `while expr {stmt}`
/// Error Handling:
/// Err:: expected expressiong if
/// while \n expr {stmt} or while {stmt} or while (by itself)
/// Err:: expected {stmt} if
/// while expr or while expr stmt
fn parse_ternary_token_asttype_asttype(
    tree: &mut ParseTreeUnfinshed,
    operator_token_types: &[TokenType],
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
        if !AST_Node::arc_belongs_to_Token_type(tree[i].clone(), operator_token_types) {
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
