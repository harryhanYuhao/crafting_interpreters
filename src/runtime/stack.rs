/// A mock stack implemented for lox language. 
/// The stack is Vec<HashMap<String, LoxVariable>>, ie, it is a vector of hashmap from string to
/// LoxVariable. LoxVariable is defined in runtime::lox_variable.
///
/// HashMap is used for quick storage and retrieval. 
///
/// Vec is used for scope. EG
///
/// ```lox
/// print(PI) // PI is in std::math scope
/// {  // this is a new scope
///     var PI = 3.14 // PI is in this scope
/// }  // end of scope
/// ```
///
/// Each scope is a hashmap. When entering a new scope, a new hashmap is pushed into the vector.
/// When leaving a scope, the last hashmap is popped. 
///
/// When variable is to be retreived, the newest scope (stack[-1]) is checked first. If not found, it will search in the previous scope
///
/// Stack is a automatically inited when calling Stack::stack(). 
/// It will not be inited before the first call to Stack::stack().
///
/// standard library exports the function lox_std::get_std() -> Vec<LoxVariable> that returns all the lox variable in the std.
///stack::Stack::init() call this function and append all into the std stack
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use log::debug;

use super::lox_variable::{LoxVariable, LoxVariableType};
use crate::runtime::lox_std::get_std;

// I think using static reference to stak make sense and is the most easy to implement
lazy_static! {
    static ref STACK: Arc<Mutex<Stack>> = Arc::new(Mutex::new(Stack { content: vec![] }));
    static ref STACK_INIT: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

/// Implementing a mock stack
///
/// At the start of the program, a buildtin variable map shall be pushed to stack.
/// This is the contenn[0].
///
/// Upon each new scope (that is, within each {}),
/// a new map is created and pushed into the stack.content.
///
/// Any new variables in the scope are stored in the newest map.
///
/// Upon leaving a scope, the last map of stack.content is popped.
#[derive(Debug)]
pub(crate) struct Stack {
    content: Vec<HashMap<String, LoxVariable>>,
}

impl Stack {
    pub(crate) fn pop_scope(&mut self) -> Option<HashMap<String, LoxVariable>> {
        self.content.pop()
    }

    pub(crate) fn new_scope(&mut self) {
        self.content.push(HashMap::new());
    }

    pub(crate) fn push(&mut self, v: LoxVariable) {
        // TODO: ERROR HANDLING
        if v.is_rvalue() {
            return;
        }

        let last_idx = self.content.len() - 1;
        let hashmap = &mut (self.content[last_idx]);
        hashmap.insert(v.get_identifier().unwrap().to_string(), v);
    }

    pub(crate) fn get(&self, identifier: &str) -> Option<&LoxVariable> {
        for maps in self.content.iter().rev() {
            match maps.get(identifier) {
                Some(a) => return Some(a),
                None => {}
            }
        }
        None
    }

    fn init() {
        let mut stack_init = STACK_INIT.lock().unwrap();
        if *stack_init {
            return;
        }
        let mut stack = STACK.lock().unwrap();
        (*stack).new_scope();
        for i in get_std() {
            (*stack).push(i);
        }
        (*stack).new_scope();
        *stack_init = true;
    }

    // Other part of the crate call this method to obtain the stack
    pub(crate) fn stack() -> Arc<Mutex<Stack>> {
        Stack::init();

        STACK.clone()
    }
}

// This macro is solely used for getting a reference of loxvariable from the stack
// $identifier must be $str
// $variable must be &LoxVariable
// $node must be the arc_mutex_ast_node, and shall be the node you obtain $variable from.
//
// This macro assigns to $variable the content of $identifier stored in the stack.
//
// If $variable is not found, return ErrorLox based on $node
macro_rules! stack_get {
    ($variable:expr, $identifier:expr, $node:expr) => {
        let __stack = crate::runtime::stack::Stack::stack();
        let __stack = __stack.lock().unwrap();
        match __stack.get($identifier) {
            None => {
                return Err(crate::ErrorLox::from_arc_mutex_ast_node(
                    $node.clone(),
                    &format!("No variable named '{}' found in stack", $identifier),
                ));
            }
            Some(a) => {
                $variable = a;
            }
        }
    };
}
