use crate::interpreter::AST_Node::AST_Node;
use std::sync::{Arc, Mutex};

pub enum VariableType {
    NUMBER(f64),
    BOOL(bool),
    STRING(String),
    FUNCTION(fn(Arc<Mutex<AST_Node>>) -> Variable),
}

// a function is also considered a variable
pub struct Variable {
    identifier: Option<String>,
    variable_type: VariableType,
    content: Option<Arc<Mutex<AST_Node>>>,
}

impl Variable {
    pub(crate) fn get_identifier(&self) -> Option<&str> {
        match &self.identifier {
            None => None,
            Some(a) => Some(a),
        }
    }

    pub(crate) fn is_lvalue(&self) -> bool {
        self.identifier.is_some()
    }

    pub(crate) fn is_rvalue(&self) -> bool {
        !self.is_lvalue()
    }
}
