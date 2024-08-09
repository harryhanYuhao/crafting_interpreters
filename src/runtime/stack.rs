use super::lox_variable::{LoxVariable, LoxVariableType};
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;

static mut STACK_INITIALISED: AtomicBool = AtomicBool::new(false);

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
struct Stack {
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

    fn init() -> Self {
        let mut stack = Stack { content: vec![] };
        stack.new_scope();
        stack
    }


    // Other part of the crate call this method to obtain the stack
    // pub(crate) fn stack() -> &Self {
    //
    // }
}
