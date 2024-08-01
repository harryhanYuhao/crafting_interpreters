//! The tree struct defined here is the abstract syntax tree
use crate::token::{self, Token, TokenType};
use rand::Rng;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ExprType {
    Normal,
    Paren,
    Negated,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum StmtType {
    Normal,
    Braced,
    Assignment,
    Declaration,
    Compound,
    If,
    Elseif,
    While,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    FunctionDef,
}

/// Potential fields are for usage during parse when the type may not be identified
/// Copulatives are tokens including `+`, `-`, `*`, `/`
/// which can be and must be placed between two expressions
#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub(crate) enum AST_Type {
    Unfinished,
    Stmt(StmtType),
    PotentialStmt,
    Expr(ExprType),
    Identifier,
    Unknown,
    Unparsed(TokenType),
    Tuple
}

impl AST_Type {
    pub(crate) fn get_all_expr() -> Vec<Self> {
        vec![
            AST_Type::Expr(ExprType::Normal),
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Expr(ExprType::Negated),
        ]
    }

    pub(crate) fn get_all_stmt() -> Vec<Self> {
        vec![
            AST_Type::Stmt(StmtType::Normal),
            AST_Type::Stmt(StmtType::Braced),
            AST_Type::Stmt(StmtType::Assignment),
            AST_Type::Stmt(StmtType::Declaration),
            AST_Type::Stmt(StmtType::Compound),
            AST_Type::Stmt(StmtType::If),
            AST_Type::Stmt(StmtType::While),
            AST_Type::Stmt(StmtType::PlusEqual),
            AST_Type::Stmt(StmtType::MinusEqual),
            AST_Type::Stmt(StmtType::StarEqual),
            AST_Type::Stmt(StmtType::SlashEqual),
        ]
    }
}

impl From<Arc<Mutex<Token>>> for AST_Type {
    fn from(s: Arc<Mutex<Token>>) -> Self {
        let res: AST_Type;
        match Token::get_token_type_from_arc(s.clone()) {
            TokenType::NUMBER => res = AST_Type::Expr(ExprType::Normal),
            TokenType::IDENTIFIER => res = AST_Type::Identifier,
            // TokenType::STMT_SEP => {
            //     res = AST_Type::Stmt(StmtType::Normal);
            // }
            TokenType::TRUE | TokenType::FALSE => {
                res = AST_Type::Expr(ExprType::Normal);
            }
            catch_all => {
                res = AST_Type::Unparsed(catch_all);
            }
        }
        res
    }
}

/// This is the grand asbtract syntax tree
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct AST_Node {
    AST_Type: AST_Type,
    token: Arc<Mutex<Token>>,
    children: Vec<Arc<Mutex<AST_Node>>>,
}

impl AST_Node {
    pub(crate) fn is_stmt(&self) -> bool {
        match self.AST_Type {
            AST_Type::Stmt(_) => return true,
            _ => return false,
        }
    }

    pub(crate) fn is_arc_mutex_stmt(input: Arc<Mutex<AST_Node>>) -> bool {
        let node = input.lock().unwrap();
        node.is_stmt()
    }

    pub(crate) fn is_compound_stmt(&self) -> bool {
        match &self.AST_Type {
            AST_Type::Stmt(inner) => match inner {
                StmtType::Compound => {
                    return true;
                }
                _ => {
                    return false;
                }
            },
            _ => return false,
        }
    }

    pub(crate) fn is_arc_mutex_compound_stmt(input: Arc<Mutex<AST_Node>>) -> bool {
        let node = input.lock().unwrap();
        AST_Node::is_compound_stmt(&node)
    }

    pub(crate) fn is_expr(&self) -> bool {
        match self.AST_Type {
            AST_Type::Expr(_) => return true,
            _ => return false,
        }
    }

    pub(crate) fn is_arc_mutex_expr(input: Arc<Mutex<AST_Node>>) -> bool {
        let node = input.lock().unwrap();
        node.is_expr()
    }

    pub(crate) fn get_AST_Type(&self) -> AST_Type {
        self.AST_Type.clone()
    }

    pub(crate) fn get_AST_Type_from_arc(arc: Arc<Mutex<AST_Node>>) -> AST_Type {
        let node = arc.lock().unwrap();
        AST_Node::get_AST_Type(&node)
    }

    pub(crate) fn get_token(&self) -> Arc<Mutex<Token>> {
        self.token.clone()
    }

    pub(crate) fn set_AST_Type(&mut self, new_type: AST_Type) {
        self.AST_Type = new_type;
    }

    pub(crate) fn set_arc_mutex_AST_Type(node: Arc<Mutex<AST_Node>>, new_type: AST_Type) {
        let mut input = node.lock().unwrap();
        input.set_AST_Type(new_type);
    }

    pub(crate) fn get_token_from_arc(arc: Arc<Mutex<AST_Node>>) -> Arc<Mutex<Token>> {
        let node = arc.lock().unwrap();
        node.token.clone()
    }

    pub(crate) fn get_token_type_from_arc(arc: Arc<Mutex<AST_Node>>) -> TokenType {
        let token = AST_Node::get_token_from_arc(arc);
        Token::get_token_type_from_arc(token)
    }

    pub(crate) fn arc_belongs_to_AST_type(arc: Arc<Mutex<AST_Node>>, types: &[AST_Type]) -> bool {
        let node_type = AST_Node::get_AST_Type_from_arc(arc);
        for i in types {
            if node_type == *i {
                return true;
            }
        }
        false
    }

    pub(crate) fn arc_belongs_to_Token_type(
        arc: Arc<Mutex<AST_Node>>,
        types: &[TokenType],
    ) -> bool {
        let node_type = AST_Node::get_token_type_from_arc(arc);
        for i in types {
            if node_type == *i {
                return true;
            }
        }
        false
    }

    pub(crate) fn append_child(&mut self, node: Arc<Mutex<AST_Node>>) {
        self.children.push(node.clone());
    }

    fn append_children(&mut self, nodes: &[Arc<Mutex<AST_Node>>]) {
        for i in nodes {
            self.children.push(i.clone());
        }
    }

    pub(crate) fn arc_mutex_append_child(input: Arc<Mutex<AST_Node>>, child: Arc<Mutex<AST_Node>>) {
        let mut input = input.lock().unwrap();
        let input = &mut input;
        input.append_child(child);
    }

    pub(crate) fn arc_mutex_append_children(
        input: Arc<Mutex<AST_Node>>,
        children: &[Arc<Mutex<AST_Node>>],
    ) {
        let mut node = input.lock().unwrap();
        node.append_children(children);
    }

    pub fn get_level(&self) -> usize {
        let children_level = self.get_children().into_iter().map(|child| {
            let child = child.lock().unwrap();
            child.get_level()
        });

        match children_level.max() {
            Some(max) => return max + 1,
            None => return 0,
        }
    }

    pub fn has_children(&self) -> bool {
        self.get_num_of_children() > 0
    }

    pub(crate) fn arc_mutex_has_children(input: Arc<Mutex<AST_Node>>) -> bool {
        let node = input.lock().unwrap();
        (&node).has_children()
    }

    fn get_num_of_children(&self) -> usize {
        self.children.len()
    }

    pub fn get_num_of_children_recurse(&self) -> usize {
        fn aux_fun(node: &AST_Node) -> usize {
            if !node.has_children() {
                return 1;
            }
            let mut res: usize = 0;
            for i in node.children.iter() {
                let child = i.lock().unwrap();
                res += aux_fun(&child);
            }
            res
        }

        if !self.has_children() {
            return 0;
        }

        return aux_fun(self);
    }

    pub fn get_children(&self) -> &[Arc<Mutex<AST_Node>>] {
        &self.children
    }

    pub fn arc_mutex_get_children(node: Arc<Mutex<AST_Node>>) -> Vec<Arc<Mutex<AST_Node>>> {
        let tmp = node.lock().unwrap();
        (&tmp).get_children().into()
    }

    pub fn random_expr(level: usize) -> Self {
        if level == 0 {
            AST_Node {
                AST_Type: AST_Type::Unknown,
                token: Arc::new(Mutex::new(Token::random())),
                children: Vec::new(),
            }
        } else {
            let num = rand::thread_rng().gen_range(1..=3);
            let children: Vec<Arc<Mutex<AST_Node>>> = (0..num)
                .map(|_| Arc::new(Mutex::new(AST_Node::random_expr(level - 1))))
                .collect();

            AST_Node {
                AST_Type: AST_Type::Unknown,
                token: Arc::new(Mutex::new(Token::random())),
                children,
            }
        }
    }

    pub(crate) fn new(AST_Type: AST_Type, token: Token) -> Self {
        AST_Node {
            AST_Type,
            token: token.into(),
            children: Vec::new(),
        }
    }

    pub(crate) fn dummy_node(AST_Type: AST_Type) -> Self {
        AST_Node {
            AST_Type,
            token: Token::dummy().into(),
            children: Vec::new(),
        }
    }
}

impl fmt::Display for AST_Node {
    // recursively print a tree
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // For better manuevour, this auxillary function gets the representation of the
        // AST_Node recursively
        fn recurse_print(input: &AST_Node) -> Vec<String> {
            let mut res: Vec<String> = vec![];
            // the content of this node
            res.push(format!(
                "{:?}      AST_Type::{:?}",
                input.token.lock().unwrap(),
                input.AST_Type
            ));

            // append the string representation of the children
            // content of each child are placed in separate lines
            let num_of_children = input.get_num_of_children();
            let children = input.get_children();

            for i in 0..num_of_children {
                let node = children[i].clone();
                let node = &(node.lock().unwrap());
                for (j, content) in recurse_print(node).iter().enumerate() {
                    let padding: String;
                    if j == 0 {
                        padding = String::from(" |-");
                    } else if i + 1 == num_of_children {
                        padding = String::from("   ");
                    } else {
                        padding = String::from(" | ");
                    }
                    res.push(padding + content);
                }
            }

            res
        }

        // a vector of string. Each one represents a new line
        let ret_vec = recurse_print(self);
        let mut ret_str: String = String::new();
        // convert the vector of strings into one string
        for i in ret_vec.iter() {
            ret_str.push_str(i);
            ret_str.push_str("\n");
        }
        // no need to check if empty. pop do nothing if empty
        ret_str.pop(); // remove the last newline
        write!(f, "{}", ret_str)
    }
}

/// Convert Arc<Mutex<Token>> into AST_Node with interpreted AST_Type
impl From<Arc<Mutex<Token>>> for AST_Node {
    fn from(s: Arc<Mutex<Token>>) -> AST_Node {
        AST_Node {
            AST_Type: AST_Type::from(s.clone()),
            token: s,
            children: Vec::new(),
        }
    }
}

impl Into<Arc<Mutex<AST_Node>>> for AST_Node {
    fn into(self) -> Arc<Mutex<AST_Node>> {
        Arc::new(Mutex::new(self))
    }
}

pub fn get_AST_Type_from_arc(input: Arc<Mutex<AST_Node>>) -> AST_Type {
    let node = input.lock().unwrap();
    node.AST_Type.clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn random_generating_AST_Node() {
        // random_expr generate 3^level number of nodes
        let level: Vec<usize> = vec![0, 1, 2, 3, 10, 16];
        for i in level {
            let res = AST_Node::random_expr(i);
            assert_eq!(res.get_level(), i);
        }
    }

    #[test]
    fn AST_display() {
        println!("{}", AST_Node::random_expr(5));
    }

    #[test]
    fn num_terminal_nodes() {
        // for now has to be checked manually
        let node = AST_Node::random_expr(3);
        println!("{}", node);
        println!("{}", node.get_num_of_children_recurse())
    }
}
