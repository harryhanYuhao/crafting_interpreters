//! The purpose of the parser is to parse the token vector and return abstract syntax tree.
//! The token vectors is generated by the scanner

use crate::token::TokenArcVec;
use crate::AST_Node::AST_Node;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::iter::FromIterator;
use std::sync::{Arc, Mutex};

pub struct ParseTreeUnfinshed {
    content: Vec<Arc<Mutex<AST_Node>>>,
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

pub enum ParseState {
    Finished(Arc<Mutex<AST_Node>>),
    Unfinished,
    Err(Box<dyn Error>),
}

impl fmt::Debug for ParseTreeUnfinshed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for i in &self.content {
            let item = i.clone();
            let item = item.lock().unwrap();
            res.push_str(&format!("{}", item));
            res.push_str("\n");
        }
        // remove the extra newline
        res.remove(res.len() - 1);
        write!(f, "{}", res)
    }
}

pub fn parse(tokens: &TokenArcVec, tree: &mut ParseTreeUnfinshed) -> ParseState {
    let mut input_list = ParseTreeUnfinshed::from(tokens);
    input_list.push(Arc::new(Mutex::new(AST_Node::random_expr(1))));
    println!("{:?}", input_list);
    ParseState::Unfinished
}
