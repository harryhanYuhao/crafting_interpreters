use crate::err_lox::ErrorLox;
use crate::interpreter::AST_Node::{AST_Node, AST_Type, StmtType};
use crate::runtime::{self, stack};
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    lexemes: Vec<String>,
    content: Arc<Mutex<AST_Node>>,
}

impl LoxFunction {
    pub(crate) fn from_ast(
        tuple: Arc<Mutex<AST_Node>>,
        execute_block: Arc<Mutex<AST_Node>>,
    ) -> Result<Self, ErrorLox> {
        AST_Node::error_handle_check_type_arc(
            tuple.clone(),
            AST_Type::Tuple, 
            "expected tuple or identifer for function definition (LoxFunction::from_ast)",
        )?;
        AST_Node::error_handle_check_type_arc(
            execute_block.clone(),
            AST_Type::Stmt(StmtType::Braced),
            "expected braced stmt for function definition",
        )?;

        let mut lexemes: Vec<String> = vec![];
        let children_tuple = AST_Node::arc_mutex_get_children(tuple.clone());
        for i in children_tuple {
            lexemes.push(AST_Node::get_token_lexeme_arc_mutex(i.clone()));
        }

        Ok(LoxFunction {
            lexemes,
            content: execute_block,
        })
    }

    fn get_lexeme(&self) -> &[String] {
        &(self.lexemes)
    }

    fn get_lexeme_length(&self) -> usize {
        self.lexemes.len()
    }

    fn get_content(&self) -> Arc<Mutex<AST_Node>> {
        self.content.clone()
    }
}

#[derive(Debug, Clone)]
pub enum LoxVariableType {
    NUMBER(f64),
    BOOL(bool),
    STRING(String),
    #[allow(non_camel_case_types)]
    // STD_Function, the input is expected to be a tuple
    STD_FUNCTION(fn(&LoxVariable) -> LoxVariable),
    #[allow(non_camel_case_types)]
    LOX_FUNCTION(LoxFunction),
    TUPLE(Vec<Box<LoxVariable>>),
    NONE,
}

impl fmt::Display for LoxVariableType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res: String;
        match self {
            LoxVariableType::NUMBER(n) => {
                res = format!("NUMBER: {n}");
            }
            LoxVariableType::BOOL(n) => {
                res = format!("BOOL: {n}");
            }
            LoxVariableType::STRING(n) => {
                res = format!("STRING: {n}");
            }
            LoxVariableType::STD_FUNCTION(_) => {
                res = format!("STD FUNCTION");
            }
            LoxVariableType::LOX_FUNCTION(_) => {
                res = format!("LOX FUNCTION");
            }
            // TODO: what is a good tuple display?
            LoxVariableType::TUPLE(_) => {
                res = format!("TUPLE");
            }
            LoxVariableType::NONE => {
                res = format!("NONE");
            }
        }
        write!(f, "{res}")
    }
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

    pub(crate) fn set_identifier(&mut self, identifier: String) {
        self.identifier = Some(identifier);
    }

    pub(crate) fn get_ref_node(&self) -> Option<Arc<Mutex<AST_Node>>> {
        self.ref_node.clone()
    }

    pub(crate) fn set_ref_node(&mut self, ref_node: Arc<Mutex<AST_Node>>) {
        self.ref_node = Some(ref_node);
    }

    pub(crate) fn get_type(&self) -> LoxVariableType {
        self.variable_type.clone()
    }

    pub(crate) fn set_type(&mut self, variable_type: LoxVariableType) {
        self.variable_type = variable_type;
    }

    pub(crate) fn get_content(&self) -> Option<Arc<Mutex<AST_Node>>> {
        self.ref_node.clone()
    }

    /// Return some(len) if the variable is a tuple, len is the length of the tuple,
    /// Return none if the variable is not a tuple
    pub(crate) fn get_tuple_length(&self) -> Option<usize> {
        match &self.variable_type {
            LoxVariableType::TUPLE(vec) => {
                let mut length = 0;
                for i in vec.iter() {
                    match i.get_type() {
                        LoxVariableType::NONE => {}
                        _ => {
                            length += 1;
                        }
                    }
                }
                return Some(length);
            }
            _ => {
                return None;
            }
        }
    }

    /// Return some(vec) if the variable is a tuple, vec holding the pointer to LoxVariables
    /// Return none if the variable is not a tuple
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

    pub(crate) fn run_std_function(&self, input: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
        if !input.is_tuple() {
            return Err(ErrorLox::from_lox_variable(input, "LoxVariable::run_std_function called with non tuple argument, likely an internal error"));
        }
        let inner_fn = match &self.variable_type {
            LoxVariableType::STD_FUNCTION(f) => *f,
            _ => {
                return Err(ErrorLox::from_lox_variable(self, "LoxVariable::run_std_function called on a non std function, likely an internal error")
                    );
            }
        };
        Ok(inner_fn(input))
    }

    pub(crate) fn run_lox_function(&self, input: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
        if !input.is_tuple() {
            return Err(ErrorLox::from_lox_variable(input, "LoxVariable::run_std_function called with non tuple argument, likely an internal error"));
        }
        let lox_fn = match &self.variable_type {
            LoxVariableType::LOX_FUNCTION(f) => f,
            _ => {
                return Err(ErrorLox::from_lox_variable(self, "LoxVariable::run_std_function called on a non std function, likely an internal error")
                )
            }
        };
        let input_length = input.get_tuple_length().unwrap();
        let expected_length = lox_fn.get_lexeme_length();

        if input_length != expected_length {
            return Err(ErrorLox::from_lox_variable(
                input,
                &format!("Expected {expected_length} inputs, found {input_length}. LoxVariable::run_lox_function"),
            ));
        }

        let lexemes = lox_fn.get_lexeme();
        let input_content = input.get_tuple_content().unwrap();
        for i in 0..lexemes.len() {
            let tmp = LoxVariable::new(
                Some(lexemes[i].clone()),
                input_content[i].get_type(),
                input_content[i].get_ref_node(),
            );
            stack::stack_push(tmp);
        }
        runtime::run(lox_fn.get_content())
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
            None => id = "RVALUE".into(),
            Some(s) => id = s.clone(),
        }
        write!(f, "{:<10}: {}", id, self.variable_type)
    }
}
