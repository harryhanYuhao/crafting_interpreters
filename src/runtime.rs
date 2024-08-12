//  DEBUG:
use log::{debug, error};
pub mod lox_std;
pub mod lox_variable;
#[macro_use]
pub mod stack;

use crate::err_lox::ErrorLox;
use crate::interpreter::token::{Token, TokenType};
use crate::interpreter::AST_Node::{AST_Node, AST_Type, ExprType, StmtType};
use lox_variable::{LoxVariable, LoxVariableType};
use std::sync::{Arc, Mutex};

fn lox_add(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    match left.get_type() {
        LoxVariableType::NUMBER(l) => {
            if !right.is_number() {
                return Err(ErrorLox::from_lox_variable(
                    right,
                    "Expected NUMBER type for right operand",
                ));
            } else {
                let num = right.get_number();
                // SUCCESS CASE
                return Ok(LoxVariable::new(
                    None,
                    LoxVariableType::NUMBER(l + num),
                    None,
                ));
            }
        }
        LoxVariableType::STRING(l) => {
            if !right.is_string() {
                return Err(ErrorLox::from_lox_variable(
                    right,
                    "Expected STRING type for right operand",
                ));
            } else {
                let r = right.get_string();
                // SUCCESS CASE
                return Ok(LoxVariable::new(
                    None,
                    LoxVariableType::STRING(l + &r),
                    None,
                ));
            }
        }
        _ => {
            return Err(ErrorLox::from_lox_variable(
                left,
                "Expected NUMBER or STRING type for left operand",
            ));
        }
    }
}

fn lox_minus(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    match left.get_type() {
        LoxVariableType::NUMBER(l) => {
            if !right.is_number() {
                return Err(ErrorLox::from_lox_variable(
                    right,
                    "Expected NUMBER type for right operand",
                ));
            } else {
                // SUCCESS CASE
                let num = right.get_number();
                return Ok(LoxVariable::new(
                    None,
                    LoxVariableType::NUMBER(l - num),
                    None,
                ));
            }
        }
        _ => {
            return Err(ErrorLox::from_lox_variable(
                left,
                "Expected NUMBER type for left operand",
            ));
        }
    }
}

fn lox_multiply(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    match left.get_type() {
        LoxVariableType::NUMBER(l) => {
            if !right.is_number() {
                return Err(ErrorLox::from_lox_variable(
                    right,
                    "Expected NUMBER type for right operand",
                ));
            } else {
                // SUCCESS CASE
                let num = right.get_number();
                return Ok(LoxVariable::new(
                    None,
                    LoxVariableType::NUMBER(l * num),
                    None,
                ));
            }
        }
        _ => {
            return Err(ErrorLox::from_lox_variable(
                left,
                "Expected NUMBER type for left operand",
            ));
        }
    }
}

fn lox_divide(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    match left.get_type() {
        LoxVariableType::NUMBER(l) => {
            if !right.is_number() {
                return Err(ErrorLox::from_lox_variable(
                    right,
                    "Expected NUMBER type for right operand",
                ));
            } else {
                // SUCCESS CASE
                let num = right.get_number();
                return Ok(LoxVariable::new(
                    None,
                    LoxVariableType::NUMBER(l / num),
                    None,
                ));
            }
        }
        _ => {
            return Err(ErrorLox::from_lox_variable(
                left,
                "Expected NUMBER type for left operand",
            ));
        }
    }
}

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
            // TODO: do we need run here?
            return eval_expr(children[0].clone());
        }
        2 => {
            match AST_Node::get_token_type_from_arc(node.clone()) {
                TokenType::PLUS => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_add(&left, &right);
                }
                TokenType::MINUS => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_minus(&left, &right);
                }
                TokenType::STAR => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_multiply(&left, &right);
                }
                TokenType::SLASH => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_divide(&left, &right);
                }
                _ => {
                    // return Err(ErrorLox::from_arc_mutex_ast_node(node.clone(), "Expected MINUS token"));
                }
            }
        }
        _ => {}
    }
    Ok(LoxVariable::empty())
}

// the input node shall be expr(function)
fn eval_expr_function(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    if AST_Node::get_AST_Type_from_arc(node.clone()) != AST_Type::Expr(ExprType::Function) {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "eval_expr_function called on non-function, likely internal error",
        ));
    }
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() != 1 {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expected only one children, Likely a parsing error",
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

    // DEBUG: ERROR Handling
    // let error = ErrorLox::from_lox_variable(&function_input, "aaa");
    // println!("{error:?}");
    // error.panic();

    // function input must be tuple
    let function_input = function_input.to_tuple();

    let lexeme = AST_Node::get_token_lexeme_arc_mutex(node.clone());
    let function: &LoxVariable;
    stack_get!(function, &lexeme, node);

    let inner_function = function.get_function();
    Ok(inner_function(&function_input))
}

fn eval_expr_paren(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    // By parsing rule all expr(paren) will have at most one child, and the
    // child shall be expression
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() == 0 {
        return Ok(LoxVariable::empty_from_arc_mutex_ast_node(node.clone()));
    } else if children.len() == 1 {
        let a = eval_expr(children[0].clone());
        //     // DEBUG: line
        // match &a {
        //     Ok(o) => {
        //         // println!("{o}");
        //     }
        //     Err(e) => {}
        // }
        return a;
    } else {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expr(Paren) has more than one children; likely a parsing error",
        ));
    }
}
fn eval_expr(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    match AST_Node::get_AST_Type_from_arc(node.clone()) {
        AST_Type::Expr(ExprType::Normal) => {
            return eval_expr_normal(node.clone());
        }
        AST_Type::Expr(ExprType::Function) => {
            return eval_expr_function(node.clone());
        }
        AST_Type::Expr(ExprType::Paren) => {
            return eval_expr_paren(node.clone());
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
