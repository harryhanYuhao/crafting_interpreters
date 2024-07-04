//! The tree struct defined here is the abstract syntax tree
use crate::token;
use rand::Rng;
use std::error::Error;
use std::fmt::{self};
use std::sync::{Arc, Mutex};
use token::{Token, TokenType};

/// This is the grand asbtract syntax tree
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub struct AST_Node {
    token: Arc<Mutex<Token>>,
    children: Vec<Arc<Mutex<AST_Node>>>,
}

impl AST_Node {
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

    pub fn get_num_of_children(&self) -> usize {
        self.children.len()
    }

    pub fn get_children(&self) -> &[Arc<Mutex<AST_Node>>] {
        &self.children
    }

    pub fn from_arc_mut_token(token: Arc<Mutex<Token>>) -> Self {
        AST_Node {
            token,
            children: Vec::new(),
        }
    }

    pub fn random_expr(level: usize) -> Self {
        if level == 0 {
            AST_Node {
                token: Arc::new(Mutex::new(Token::random())),
                children: Vec::new(),
            }
        } else {
            let num = rand::thread_rng().gen_range(1..=3);
            let children: Vec<Arc<Mutex<AST_Node>>> = (0..num)
                .map(|_| Arc::new(Mutex::new(AST_Node::random_expr(level - 1))))
                .collect();

            AST_Node {
                token: Arc::new(Mutex::new(Token::random())),
                children,
            }
        }
    }

    pub fn new(token: Arc<Mutex<token::Token>>) -> Self {
        AST_Node {
            token,
            children: Vec::new(),
        }
    }

    pub fn new_terminal_node(token: Arc<Mutex<token::Token>>) -> Self {
        AST_Node {
            token,
            children: Vec::new(),
        }
    }
}

impl fmt::Display for AST_Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recurse_print(input: &AST_Node) -> Vec<String> {
            let mut res: Vec<String> = vec![];
            res.push(format!("{:?}", input.token.lock().unwrap()));
            let num_of_children = input.get_num_of_children();
            let children = input.get_children();

            let bound = match num_of_children > 1 {
                true => num_of_children - 1,
                false => 0,
            };
            for i in 0..bound {
                let node = children[i].clone();
                let node = &(node.lock().unwrap());
                for (j, content) in recurse_print(node).iter().enumerate() {
                    let mut padding = String::new();
                    if j == 0 {
                        padding.push_str(" |-".into());
                    } else {
                        padding.push_str(" | ".into());
                    }
                    padding.push_str(content);
                    res.push(padding);
                }
            }

            if num_of_children >= 1 {
                let node = Arc::clone(&children[num_of_children - 1]);
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
}
