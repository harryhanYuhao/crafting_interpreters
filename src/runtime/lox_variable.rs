use crate::interpreter::AST_Node::AST_Node;
use std::fmt;
use std::sync::{Arc, Mutex};

//
#[derive(Debug, Clone)]
pub enum LoxVariableType {
    NUMBER(f64),
    BOOL(bool),
    STRING(String),
    #[allow(non_camel_case_types)]
    // STD_Function, the input is expected to be a tuple
    STD_FUNCTION(fn(&LoxVariable) -> LoxVariable),
    TUPLE(Vec<Box<LoxVariable>>),
    NONE,
}

// a function is also considered a variable
#[derive(Debug, Clone)]
pub struct LoxVariable {
    identifier: Option<String>,
    variable_type: LoxVariableType,
    // ref_node has two purposes:
    // 1. get and parse the lexeme 2. error handling
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

    pub(crate) fn get_identifier(&self) -> Option<String> {
        match &self.identifier {
            None => None,
            Some(a) => Some(a.clone()),
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

    /// Return some(len) if the variable is a tuple, len is the length of the tuple,
    /// Return none if the variable is not a tuple
    pub(crate) fn get_tuple_length(&self) -> Option<usize> {
        match &self.variable_type {
            LoxVariableType::TUPLE(vec) => {
                return Some(vec.len());
            }
            _ => {
                return None;
            }
        }
    }

    pub(crate) fn get_tuple_content(&self) -> Option<Vec<Box<LoxVariable>>> {
        match &self.variable_type {
            LoxVariableType::TUPLE(vec) => {
                return Some(vec.clone());
            }
            _ => {
                return None;
            }
        }
    }

    pub(crate) fn is_lvalue(&self) -> bool {
        self.identifier.is_some()
    }

    pub(crate) fn is_rvalue(&self) -> bool {
        !self.is_lvalue()
    }

    pub(crate) fn is_number(&self) -> bool {
        match &self.variable_type {
            LoxVariableType::NUMBER(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_number(&self) -> f64 {
        match &self.variable_type {
            LoxVariableType::NUMBER(n) => *n,
            _ => panic!("LoxVariable::get_number called on a none number: Internal error"),
        }
    }

    pub(crate) fn is_bool(&self) -> bool {
        match &self.variable_type {
            LoxVariableType::BOOL(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_bool(&self) -> bool {
        match &self.variable_type {
            LoxVariableType::BOOL(b) => *b,
            _ => panic!("LoxVariable::get_bool called on a none bool: Internal error"),
        }
    }

    pub(crate) fn is_string(&self) -> bool {
        match &self.variable_type {
            LoxVariableType::STRING(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_string(&self) -> String {
        match &self.variable_type {
            LoxVariableType::STRING(s) => s.clone(),
            _ => panic!("LoxVariable::get_string called on a none string: Internal error"),
        }
    }

    pub(crate) fn is_function(&self) -> bool {
        match &self.variable_type {
            LoxVariableType::STD_FUNCTION(_) => true,
            _ => false,
        }
    }

    pub(crate) fn get_function(&self) -> fn(&LoxVariable) -> LoxVariable {
        match &self.variable_type {
            LoxVariableType::STD_FUNCTION(f) => {
                return *f;
            }
            _ => {
                panic!("LoxVariable::get_function called on a none function: Internal error");
            }
        }
    }
    pub(crate) fn is_tuple(&self) -> bool {
        match &self.variable_type {
            LoxVariableType::TUPLE(_) => true,
            _ => false,
        }
    }

    pub(crate) fn is_none(&self) -> bool {
        match &self.variable_type {
            LoxVariableType::NONE => true,
            _ => false,
        }
    }

    pub(crate) fn empty() -> Self {
        LoxVariable {
            identifier: None,
            variable_type: LoxVariableType::NONE,
            ref_node: None,
        }
    }

    pub(crate) fn empty_from_arc_mutex_ast_node(node: Arc<Mutex<AST_Node>>) -> Self {
        LoxVariable {
            identifier: None,
            variable_type: LoxVariableType::NONE,
            ref_node: Some(node),
        }
    }

    pub(crate) fn to_tuple(&self) -> Self {
        match self.variable_type {
            LoxVariableType::TUPLE(_) => {
                return self.clone();
            }
            _ => {
                return LoxVariable {
                    identifier: None,
                    variable_type: LoxVariableType::TUPLE(vec![Box::new(self.clone())]),
                    // WARNING: MAY have circular dependency
                    ref_node: self.ref_node.clone(),
                };
            }
        }
    }
}

impl fmt::Display for LoxVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id: String;
        match &self.identifier {
            None => id = "NONAME".into(),
            Some(s) => id = s.clone(),
        }
        write!(f, "{}: {:?}", id, self.variable_type)
    }
}
