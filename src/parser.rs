use crate::token;
use std::error::Error;
use std::fmt;
use std::sync::{Arc, Mutex};
use token::{Token, TokenType};

#[derive(Debug)]
pub enum TreeType {
    EXPR, // expression
    STMT, // statement
}

#[derive(Debug)]
pub struct Tree {
    // expr: short for expression
    // If it is not an expression, it is a statment.
    tree_type: TreeType,
    token: Arc<Mutex<token::Token>>,
    left: Option<Arc<Mutex<Tree>>>,
    right: Option<Arc<Mutex<Tree>>>,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recurse_print(tree: &Tree, padding: usize) -> Vec<String> {
            let mut res: Vec<String> = vec![];
            res.push(format!("{:?}", tree.token.lock()));
            if tree.left.is_some() {
                let left_tmp = tree.left.as_ref().unwrap().clone();
                let left_lock = &(left_tmp.lock().unwrap());
                let tmp = recurse_print(left_lock, padding + 1);
                let mut i = 0;
                loop {
                    if i == tmp.len() {
                        break;
                    }
                    if i == 0 {
                        // res.push(" | ".into());
                        let mut to_push = String::from(" |-");
                        to_push.push_str(&tmp[i]);
                        res.push(to_push);
                    }
                    let mut to_push = String::new();
                    if tree.right.is_some() {
                        to_push.push_str(" | ".into());
                    } else {
                        to_push.push_str("   ".into());
                    }
                    i += 1;
                }
            };
            if tree.right.is_some() {
                let left_tmp = tree.right.as_ref().unwrap().clone();
                let left_lock = &(left_tmp.lock().unwrap());
                let tmp = recurse_print(left_lock, padding + 1);
                let mut i = 0;
                loop {
                    if i == tmp.len() {
                        break;
                    }
                    if i == 0 {
                        // res.push(" | ".into());
                        let mut to_push = String::from(" |-");
                        to_push.push_str(&tmp[i]);
                        res.push(to_push);
                    }
                    let mut to_push = String::new();
                    to_push.push_str("   ".into());
                    i += 1;
                }
            }
            res
        }

        // a vector of string. Each one represents a new line
        let ret_vec = recurse_print(self, 0);
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
    // only eval expression
    fn eval(&self) -> Result<f64, Box<dyn Error>> {
        if matches!(self.tree_type, TreeType::STMT) {
            return Err("Not An Expression".into());
        }

        let token = self.token.lock().unwrap();
        match token.token_type {
            TokenType::NUMBER => {
                return Ok(token.lexeme.parse::<f64>()?);
            }
            TokenType::PLUS => {
                let mut res: f64 = 0.0;
                match &self.left {
                    Some(child) => {
                        res += child.lock().unwrap().eval()?;
                    }
                    None => return Err("No Child_1!".into()),
                };
                match &self.right {
                    Some(child) => {
                        res += child.lock().unwrap().eval()?;
                    }
                    None => return Err("No Child_1!".into()),
                };
                return Ok(res);
            }
            _ => {}
        }
        Err("None".into())
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
            tree_type: TreeType::EXPR,
            token,
            left: None,
            right: None,
        }

    }

    fn parse(tokens: &[Arc<Mutex<Token>>]) -> Option<Arc<Mutex<Tree>>> {
        let mut left: Option<Tree> = None;
        for (i, token) in tokens.iter().enumerate() {
            match token.lock().unwrap().token_type {
                TokenType::NUMBER => {

                }
                _ => {}
            }
        }
        Some(Arc::new(Mutex::new(left.unwrap())))
    }
}
