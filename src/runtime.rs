//  DEBUG:
use log::{debug, error};
pub mod lox_std;
pub mod lox_variable;
pub mod stack;

use crate::err_lox::ErrorLox;
use crate::interpreter::token::{Token, TokenType};
use crate::interpreter::AST_Node::{AST_Node, AST_Type, ExprType, StmtType};
use lox_std::io::print_lox;
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
        TokenType::STRING => {
            return Ok(LoxVariable::new(
                None,
                LoxVariableType::STRING(token.get_lexeme().clone()),
                None,
            ))
        }
        TokenType::TRUE => return Ok(LoxVariable::new(None, LoxVariableType::BOOL(true), None)),
        TokenType::FALSE => return Ok(LoxVariable::new(None, LoxVariableType::BOOL(false), None)),
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
        AST_Type::Expr(ExprType::Function) => {
            let children = AST_Node::arc_mutex_get_children(node.clone());
            if children.len() != 1 {
                return Err(ErrorLox::from_arc_mutex_ast_node(
                    node.clone(),
                    "Expected only on children, Likely a parsing error",
                ));
            } else if AST_Node::get_AST_Type_from_arc(children[0].clone())
                != AST_Type::Expr(ExprType::Paren)
            {
                return Err(ErrorLox::from_arc_mutex_ast_node(
                    node.clone(),
                    "Expected expr(paren), likely a parsing error",
                ));
            }
            let function_input = eval_expr(children[0].clone())?;
            let function_input = function_input.to_tuple();

            let function: &LoxVariable;
            let stack = stack::Stack::stack();
            let stack = stack.lock().unwrap();
            match stack.get("print") {
                None => {
                    return Err(ErrorLox::from_arc_mutex_ast_node(
                        node.clone(),
                        &format!("No {} found in stack", "print"),
                    ));
                }
                Some(a) => {
                    function = a;
                }
            }

            match function.get_function() {
                None => {
                    return Err(ErrorLox::from_arc_mutex_ast_node(
                        node.clone(),
                        &format!("{} is not a function", "print"),
                    ));
                }
                Some(f) => {
                    return Ok(f(&function_input));
                }
            }
        }
        AST_Type::Expr(ExprType::Paren) => {
            // By parsing rule all expr(paren) will have at most one child, and the
            // child shall be expression
            let children = AST_Node::arc_mutex_get_children(node.clone());
            if children.len() == 0 {
                return Ok(LoxVariable::empty_from_arc_mutex_ast_node(node.clone()));
            } else if children.len() == 1 {
                let a = eval_expr(children[0].clone());
                match &a {
                    // DEBUG: line
                    Ok(o) => {
                        // println!("{o}");
                    }
                    Err(e) => {}
                }
                return a;
            } else {
                return Err(ErrorLox::from_arc_mutex_ast_node(
                    node.clone(),
                    "Expr(Paren) has more than one children; likely a parsing error",
                ));
            }
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
        AST_Type::Expr(ExprType::Normal)
        | AST_Type::Expr(ExprType::Paren)
        | AST_Type::Expr(ExprType::Negated)
        | AST_Type::Expr(ExprType::Function) => {
            return eval_expr(tree.clone());
        }
        res => {
            println!("res: {:?}", res);
        }
    }
    Ok(LoxVariable::empty())
}
