use crate::token;
use std::error::Error;
use std::fmt::{self, write};
use std::sync::{Arc, Mutex};
use token::{Token, TokenType};

// EXPR, // expression
// STMT, // statement
#[derive(Debug)]
pub enum TreeType {
    NUMBER,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    PARENTHESIS,
    UNDEFINED,
}

// tree option
pub type TreeOP = Option<Arc<Mutex<Tree>>>;

#[derive(Debug)]
pub struct Tree {
    // expr: short for expression
    // If it is not an expression, it is a statment.
    tree_type: TreeType,
    token: Arc<Mutex<token::Token>>,
    left: TreeOP,
    right: TreeOP,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recurse_print(tree: &Tree) -> Vec<String> {
            let mut res: Vec<String> = vec![];
            res.push(format!("{:?}", tree.token.lock().unwrap()));

            if tree.left.is_some() {
                let node = tree.left.as_ref().unwrap().clone();
                let node = &(node.lock().unwrap());
                for (i, content) in recurse_print(node).iter().enumerate() {
                    let mut padding = String::new();
                    if i == 0 {
                        padding.push_str(" |-".into());
                    } else {
                        if tree.right.is_some() {
                            padding.push_str(" | ".into());
                        } else {
                            padding.push_str("   ".into());
                        }
                    }
                    padding.push_str(content);
                    res.push(padding);
                }
            };

            if tree.right.is_some() {
                let node = tree.right.as_ref().unwrap().clone();
                let node = &(node.lock().unwrap());
                for (i, content) in recurse_print(node).iter().enumerate() {
                    let mut padding = String::new();
                    if i == 0 {
                        padding.push_str(" |-".into());
                    } else {
                        padding.push_str("   ".into());
                    }
                    padding.push_str(content);
                    res.push(padding);
                }
            };
            res
        }

        // a vector of string. Each one represents a new line
        let ret_vec = recurse_print(self);
        let mut ret_str: String = String::new();
        for i in ret_vec.iter() {
            ret_str.push_str(i);
            ret_str.push_str("\n");
        }
        // no need to check if empty. pop return none if empty
        ret_str.pop(); // remove the last newline
        write!(f, "{}", ret_str)
    }
}

impl Tree {
    pub fn random_expr(level: usize) -> Self {
        if level == 0 {
            Tree {
                tree_type: TreeType::NUMBER,
                token: Arc::new(Mutex::new(Token::random())),
                left: None,
                right: None,
            }
        } else {
            let left = Tree::random_expr(level - 1);
            let right = Tree::random_expr(level - 1);
            Tree {
                tree_type: TreeType::NUMBER,
                token: Arc::new(Mutex::new(Token::random())),
                left: Some(Arc::new(Mutex::new(left))),
                right: Some(Arc::new(Mutex::new(right))),
            }
        }
    }

    // only eval expression
    pub fn eval(&self) -> Result<f64, Box<dyn Error>> {
        // evaluating both child and return tuple
        // this function is a wrapper as the child is of type Option<Arc<Mutex<Tree>>>
        fn eval_child(node: &Tree) -> Result<(f64, f64), Box<dyn Error>> {
            if node.right.is_none() || node.left.is_none() {
                return Err("CHILD IS NODE when evaluating!".into());
            }

            let left = node.left.as_ref().unwrap();
            let left = left.lock().unwrap();
            let left = left.eval()?;

            let right = node.right.as_ref().unwrap();
            let right = right.lock().unwrap();
            let right = right.eval()?;
            Ok((left, right))
        }

        // ***********************************
        // **** START OF EXECUTION
        // ***********************************

        let token = self.token.lock().unwrap();
        let mut res: f64 = 0.0;
        match token.token_type {
            TokenType::NUMBER => {
                res = token.lexeme.parse::<f64>()?;
            }
            TokenType::PLUS => {
                let (left, right) = eval_child(self)?;
                res = left + right;
            }
            TokenType::MINUS => {
                let (left, right) = eval_child(self)?;
                res = left - right;
            }
            TokenType::STAR => {
                let (left, right) = eval_child(self)?;
                res = left * right;
            }
            TokenType::SLASH => {
                let (left, right) = eval_child(self)?;
                res = left / right;
            }
            _ => {}
        }
        Ok(res)
    }

    fn new(
        tree_type: TreeType,
        token: Arc<Mutex<token::Token>>,
        left: Option<Arc<Mutex<Tree>>>,
        right: Option<Arc<Mutex<Tree>>>,
    ) -> Self {
        Tree {
            tree_type,
            token,
            left,
            right,
        }
    }

    fn new_terminal_node(token: Arc<Mutex<token::Token>>) -> Self {
        Tree {
            tree_type: TreeType::UNDEFINED,
            token,
            left: None,
            right: None,
        }
    }
}

pub fn parse(tokens: &[Arc<Mutex<Token>>]) -> Result<Arc<Mutex<Tree>>, Box<dyn Error>> {
    let mut left: Option<Tree> = None;

    for (i, token) in tokens.iter().enumerate() {
        let token_ref = token.lock().unwrap();
    }
    Ok(Arc::new(Mutex::new(left.unwrap())))
}
