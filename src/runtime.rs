pub mod stack;
pub mod variable;
pub mod lox_std;

use crate::err_lox::ErrorLox;
use crate::interpreter::AST_Node::{AST_Node, AST_Type, StmtType};
use std::sync::{Arc, Mutex};
use variable::LoxVariable;

fn execute_compound_stmt(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    let children = AST_Node::arc_mutex_get_children(node.clone());
    for (idx, i) in children.iter().enumerate() {
        if idx == children.len() - 1 {
            return run(i.clone());
        } else {
            run(i.clone())?;
        }
    }
    Ok(LoxVariable::empty())
}

pub fn run(tree: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    match AST_Node::get_AST_Type_from_arc(tree.clone()) {
        AST_Type::Stmt(StmtType::Compound) => {
            return execute_compound_stmt(tree.clone());
        }
        res => {
            println!("{:?}", res);
        }
    }
    Ok(LoxVariable::empty())
}
