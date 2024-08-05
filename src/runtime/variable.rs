use crate::interpreter::AST_Node::AST_Node;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum LoxVariableType {
    NUMBER(f64),
    BOOL(bool),
    STRING(String),
    FUNCTION(fn(&LoxVariable) -> LoxVariable),
    NONE,
}

// a function is also considered a variable
#[derive(Debug)]
pub struct LoxVariable {
    identifier: Option<String>,
    variable_type: LoxVariableType,
    ref_node: Option<Arc<Mutex<AST_Node>>>,
}

impl LoxVariable {
    pub(crate) fn new(
        identifier: Option<String>,
        variable_type: LoxVariableType,
        ref_node: Option<Arc<Mutex<AST_Node>>>,
    ) -> Self {
        LoxVariable {
            identifier,
            variable_type,
            ref_node,
        }
    }

    pub(crate) fn get_identifier(&self) -> Option<&str> {
        match &self.identifier {
            None => None,
            Some(a) => Some(a),
        }
    }

    pub(crate) fn get_ref_node(&self) -> Option<Arc<Mutex<AST_Node>>> {
        self.ref_node.clone()
    }

    pub(crate) fn get_type(&self) -> LoxVariableType {
        self.variable_type.clone()
    }

    pub(crate) fn get_content(&self) -> Option<Arc<Mutex<AST_Node>>> {
        self.ref_node.clone()
    }

    pub(crate) fn is_lvalue(&self) -> bool {
        self.identifier.is_some()
    }

    pub(crate) fn is_rvalue(&self) -> bool {
        !self.is_lvalue()
    }

    pub(crate) fn empty() -> Self {
        LoxVariable {
            identifier: None,
            variable_type: LoxVariableType::NONE,
            ref_node: None,
        }
    }
}
