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
/// Stack is a automatically constructed by calling the init function when calling Stack::stack()
/// is first called.
///
/// standard library exports the function lox_std::get_std() -> Vec<LoxVariable> that returns all the lox variable in the std.
/// stack::Stack::init() call this function and append all into the std stack
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use log::debug;

use super::lox_variable::{LoxVariable, LoxVariableType};
use crate::interpreter::AST_Node::AST_Node;
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
    content: Vec<HashMap<String, Arc<Mutex<LoxVariable>>>>,
}

impl Stack {
    pub(crate) fn pop_scope(&mut self) -> Option<HashMap<String, Arc<Mutex<LoxVariable>>>> {
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
        hashmap.insert(
            v.get_identifier().unwrap().to_string(),
            Arc::new(Mutex::new(v)),
        );
    }

    pub(crate) fn get(&self, identifier: &str) -> Option<Arc<Mutex<LoxVariable>>> {
        for maps in self.content.iter().rev() {
            match maps.get(identifier) {
                Some(a) => return Some(a.clone()),
                None => {}
            }
        }
        None
    }

    // pub(crate) fn get_mut(&self, identifier: &str) -> Option<&mut LoxVariable> {
    //     for maps in self.content.iter().rev() {
    //         match maps.get(identifier) {
    //             Some(a) => return Some(a),
    //             None => {}
    //         }
    //     }
    //     None
    // }

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
    fn stack() -> Arc<Mutex<Stack>> {
        Stack::init();

        STACK.clone()
    }
}

pub(crate) fn stack_get_variable(
    identifier: &str,
    node: Arc<Mutex<AST_Node>>,
) -> Result<Arc<Mutex<LoxVariable>>, crate::ErrorLox> {
    let stack = Stack::stack();
    let stack = stack.lock().unwrap();
    match stack.get(identifier) {
        None => {
            return Err(crate::ErrorLox::from_arc_mutex_ast_node(
                node.clone(),
                &format!("Can not find value '{}' in scope. Variable can only be used after declaration.", identifier),
            ));
        }
        Some(a) => {
            return Ok(a.clone());
        }
    }
}

pub(crate) fn stack_push(v: LoxVariable) {
    let stack = Stack::stack();
    let mut stack = stack.lock().unwrap();
    stack.push(v);
}

pub(crate) fn stack_new_scope() {
    let stack = Stack::stack();
    let mut stack = stack.lock().unwrap();
    stack.new_scope();
}

pub(crate) fn stack_pop_scope() {
    let stack = Stack::stack();
    let mut stack = stack.lock().unwrap();
    stack.pop_scope();
}
