use std::error::Error;
use std::fmt;
use std::iter::{Enumerate, FromIterator};
use std::ops::{Index, IndexMut, Sub};
use std::sync::{Arc, Mutex};

use super::AST_Node::{AST_Node, AST_Type};
use crate::err_lox::*;
use crate::interpreter::parser::ParseState;
use crate::interpreter::token::{self, Token, TokenArcVec, TokenType};

use log::debug;
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
    pub fn get_finished_node(&self) -> Result<Option<Arc<Mutex<AST_Node>>>, ErrorLox> {
        if self.content.len() > 1 {
            return Err(ErrorLox::from_arc_mutex_ast_node(
                self[1].clone(),
                "Internal Error: Parse Tree is unfinished when calling get_finished_node",
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
    /// if mathced, return the number of nodes that matches the repetitive pattern
    ///
    /// eg:
    pub(crate) fn match_ast_repetitive_pattern(
        &mut self,
        index: usize,
        patterns: &[Vec<AST_Type>],
    ) -> (RepetitivePatternMatchingRes, usize) {
        let mut matched_num = 0;
        let pattern_length = patterns.len();
        let mut length = self.len();
        let init_len = length;
        while index + matched_num < length
            && AST_Node::arc_belongs_to_AST_type(
                self[index + matched_num].clone(),
                &patterns[matched_num % pattern_length],
            )
        {
            matched_num += 1;
            delete_stmt_sep_adjust_len!(self, index + matched_num, length);
        }

        if matched_num == 0 {
            return (RepetitivePatternMatchingRes::Nomatch, 0);
        }

        (
            RepetitivePatternMatchingRes::MatchUntil(matched_num - 1),
            init_len - length,
        )
    }

    // check if tree[i] exist (ie tree[i] is not out of bound)
    // and if tree[i] is in types
    // if not, return error
    pub(crate) fn error_handle_tree_i_is_in_types(
        &self,
        i: usize,
        types: &[AST_Type],
        err_info: &str,
    ) -> Result<(), ErrorLox> {
        let length = self.len();
        if i > length - 1 || !AST_Node::arc_belongs_to_AST_type(self[i].clone(), types) {
            let error_idx: usize;
            let error_message: String;
            if i > length - 1 {
                error_idx = length - 1;
                error_message = format!("Expected {types:?}, found nothing. {err_info}",);
            } else {
                error_message = format!(
                    "Expected {types:?}, found {:?}. {err_info}",
                    AST_Node::get_AST_Type_from_arc(self[i].clone())
                );
                error_idx = i;
            }
            return Err(ErrorLox::from_arc_mutex_ast_node(
                self[error_idx].clone(),
                &error_message,
            ));
        }
        Ok(())
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
