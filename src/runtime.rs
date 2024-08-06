//  DEBUG:
use log::{debug, error};
pub mod lox_std;
pub mod lox_variable;
pub mod stack;

use crate::err_lox::ErrorLox;
use crate::interpreter::token::{Token, TokenType};
use crate::interpreter::AST_Node::{AST_Node, AST_Type, ExprType, StmtType};
use lox_variable::{LoxVariable, LoxVariableType};
use std::sync::{Arc, Mutex};

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

fn eval_lone_expr(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() != 0 {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Internal Runtime Error: eval_lone_expr called on none lone expr",
        ));
    }

    let token = AST_Node::get_token_from_arc(node.clone());
    let token = token.lock().unwrap();
    match token.get_token_type() {
        TokenType::NUMBER => {
            let num: f64;
            match token.get_lexeme().parse() {
                Ok(n) => num = n,
                Err(e) => {
                    return Err(ErrorLox::from_token(
                        &token,
                        &format!("Failed to parse NUM!\n {e:?}"),
                    ));
                }
            }

            return Ok(LoxVariable::new(
                None,
                LoxVariableType::NUMBER(num),
                Some(node.clone()),
            ));
        }
        _ => {}
    }

    // TODO: UNFINISHED
    Ok(LoxVariable::empty())
}

fn eval_expr_normal(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    let children = AST_Node::arc_mutex_get_children(node.clone());
    match children.len() {
        0 => {
            return eval_lone_expr(node.clone());
        }
        1 => {
            return eval_lone_expr(children[0].clone());
        }
        2 => {}
        _ => {}
    }
    Ok(LoxVariable::empty())
}

fn eval_expr(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    match AST_Node::get_AST_Type_from_arc(node.clone()) {
        AST_Type::Expr(ExprType::Normal) => {
            return eval_expr_normal(node.clone());
        }
        _ => {}
    }

    let children = AST_Node::arc_mutex_get_children(node.clone());
    // debug!("{node:?}");
    if children.len() == 0 {}
    if children.len() == 1 {}
    Ok(LoxVariable::empty())
}

pub fn run(tree: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    match AST_Node::get_AST_Type_from_arc(tree.clone()) {
        AST_Type::Stmt(StmtType::Compound) => {
            return execute_compound_stmt(tree.clone());
        }
        AST_Type::Stmt(StmtType::Normal) => {
            return execute_compound_stmt(tree.clone());
        }
        AST_Type::Expr(ExprType::Normal) => {
            return eval_expr(tree.clone());
        }
        res => {
            println!("{:?}", res);
        }
    }
    Ok(LoxVariable::empty())
}
