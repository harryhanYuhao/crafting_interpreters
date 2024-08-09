use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use super::lox_variable::{LoxVariable, LoxVariableType};
use crate::runtime::lox_std::get_std;

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
            &(*stack).push(i);
        }
        *stack_init = true;
    }

    // Other part of the crate call this method to obtain the stack
    pub(crate) fn stack() -> Arc<Mutex<Stack>> {
        Stack::init();

        STACK.clone()
    }
}

// TODO: how to properly return this reference?
//
// pub(crate) fn get(id: &str) -> Option<&LoxVariable> {
//     Stack::init();
//     let stack = STACK.lock().unwrap();
//     match &stack.get(id){
//         None => {
//             return None
//         }
//         Some(a) => {
//             return Some(*a)
//         }
//     }
// }
