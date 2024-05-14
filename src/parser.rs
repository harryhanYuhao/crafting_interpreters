use crate::token;
use std::error::Error;
use std::fmt::{self, write};
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
                tree_type: TreeType::EXPR,
                token: Arc::new(Mutex::new(Token::random())),
                left: None,
                right: None,
            }
        } else {
            let left = Tree::random_expr(level - 1);
            let right = Tree::random_expr(level - 1);
            Tree {
                tree_type: TreeType::EXPR,
                token: Arc::new(Mutex::new(Token::random())),
                left: Some(Arc::new(Mutex::new(left))),
                right: Some(Arc::new(Mutex::new(right))),
            }
        }
    }

    // only eval expression
    pub fn eval(&self) -> Result<f64, Box<dyn Error>> {
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
}

pub fn parse(tokens: &[Arc<Mutex<Token>>]) -> Option<Arc<Mutex<Tree>>> {
    let mut left: Option<Tree> = None;
    for (i, token) in tokens.iter().enumerate() {
        let token_ref = token.lock().unwrap();
        match token_ref.token_type {
            TokenType::NUMBER => {
                left = Some(Tree::new_terminal_node(token.clone()));
            }
            TokenType::PLUS => {
                if tokens.len() <= i + 1 {
                    panic!("+ find at the end of sentence!");
                }
                let right = parse(&tokens[i + 1..]);
                return Some(Arc::new(Mutex::new(Tree::new(
                    TreeType::EXPR,
                    token.clone(),
                    Some(Arc::new(Mutex::new(left.unwrap()))),
                    right,
                ))));
            }
            _ => {}
        }
    }
    Some(Arc::new(Mutex::new(left.unwrap())))
}
