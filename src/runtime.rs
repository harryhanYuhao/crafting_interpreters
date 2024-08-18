//  DEBUG:
use log::{debug, error};
pub mod lox_std;
pub mod lox_variable;
#[macro_use]
pub mod stack;

use crate::err_lox::ErrorLox;
use crate::interpreter::token::{Token, TokenType};
use crate::interpreter::AST_Node::{AST_Node, AST_Type, ExprType, StmtType};
use lox_variable::{LoxFunction, LoxVariable, LoxVariableType};
use std::env::var;
use std::sync::{Arc, Mutex};

use self::stack::{stack_get_variable, stack_push};

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
                    left.get_ref_node(),
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
                    left.get_ref_node(),
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
                    left.get_ref_node(),
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
                    left.get_ref_node(),
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
                    left.get_ref_node(),
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

fn lox_modula(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
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
                    LoxVariableType::NUMBER(l % num),
                    left.get_ref_node(),
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

fn lox_negate(variable: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    match variable.get_type() {
        LoxVariableType::NUMBER(n) => {
            return Ok(LoxVariable::new(
                variable.get_identifier(),
                LoxVariableType::NUMBER(-n),
                variable.get_ref_node(),
            ))
        }
        LoxVariableType::BOOL(b) => {
            return Ok(LoxVariable::new(
                variable.get_identifier(),
                LoxVariableType::BOOL(!b),
                variable.get_ref_node(),
            ))
        }
        lox_type => {
            return Err(ErrorLox::from_lox_variable(
                variable,
                &format!("lox_negate: expected NUMBER or BOOL, found {lox_type:?}"),
            ))
        }
    }
}

fn lox_greater(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
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
                    LoxVariableType::BOOL(l > num),
                    left.get_ref_node(),
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

fn lox_greater_equal(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
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
                    LoxVariableType::BOOL(l >= num),
                    left.get_ref_node(),
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

fn lox_equal_equal(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
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
                    LoxVariableType::BOOL(l == num),
                    left.get_ref_node(),
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
                    LoxVariableType::BOOL(l == r),
                    left.get_ref_node(),
                ));
            }
        }
        LoxVariableType::BOOL(l) => {
            if !right.is_bool() {
                return Err(ErrorLox::from_lox_variable(
                    right,
                    "Expected BOOL type for right operand",
                ));
            } else {
                let r = right.get_bool();
                // SUCCESS CASE
                return Ok(LoxVariable::new(
                    None,
                    LoxVariableType::BOOL(l == r),
                    left.get_ref_node(),
                ));
            }
        }
        _ => {
            return Err(ErrorLox::from_lox_variable(
                left,
                "Expected NUMBER or STRING or BOOL type for left operand",
            ));
        }
    }
}

fn lox_less(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
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
                    LoxVariableType::BOOL(l < num),
                    left.get_ref_node(),
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

fn lox_less_equal(left: &LoxVariable, right: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
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
                    LoxVariableType::BOOL(l <= num),
                    left.get_ref_node(),
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
                Some(node.clone()),
            ))
        }
        TokenType::TRUE => {
            return Ok(LoxVariable::new(
                None,
                LoxVariableType::BOOL(true),
                Some(node.clone()),
            ))
        }
        TokenType::FALSE => {
            return Ok(LoxVariable::new(
                None,
                LoxVariableType::BOOL(false),
                Some(node.clone()),
            ))
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
            // TODO: do we need run here?
            return eval_expr(children[0].clone());
        }
        2 => {
            match AST_Node::get_token_type_from_arc(node.clone()) {
                TokenType::PLUS => {
                    let left = eval_expr(children[0].clone())?;
                    let right = eval_expr(children[1].clone())?;
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
                TokenType::PERCENT => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_modula(&left, &right);
                }
                TokenType::GREATER => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_greater(&left, &right);
                }
                TokenType::GREATER_EQUAL => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_greater_equal(&left, &right);
                }
                TokenType::EQUAL_EQUAL => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_equal_equal(&left, &right);
                }
                TokenType::BANG_EQUAL => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_negate(&lox_equal_equal(&left, &right)?);
                }
                TokenType::LESS => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_less(&left, &right);
                }
                TokenType::LESS_EQUAL => {
                    let left = eval_expr(children[0].clone()).unwrap();
                    let right = eval_expr(children[1].clone()).unwrap();
                    return lox_less_equal(&left, &right);
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

/// the input node shall be expr(function)
/// There are two kinds of function, std function and lox function
/// because lox is only an interpreter, it has to rely on native rust function in some senarios (C
/// function call, print, etc). These native functions are std function and are written purly in
/// rust, but is also stored as lox_variable as the type STD_FUNCTION in stack.
/// The lox_variable contains the function
/// pointer to the native function which is executed at the runtime.
/// The input for the function is LoxTuple. Errors (types of input, number of input, etc) are
/// handled by the rust code at runtime.
///
/// lox also has function written purly in lox. These function are represented by the type
/// LOX_FUNCTION, which contains lexemes and pointer to the block of stmt(brace) to be executed.
/// At compile time, number of lexemes are checked, and the variables are evaluated and pushed to
/// the new stack. The block of code is executed by calling run, and in the end the stack scope is
/// popped.
fn eval_expr_function(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    if AST_Node::get_AST_Type_from_arc(node.clone()) != AST_Type::Expr(ExprType::Function) {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "eval_expr_function called on non-function, likely internal error",
        ));
    }
    AST_Node::error_handle_check_children_num_and_type_arc(
        node.clone(),
        &vec![AST_Type::Expr(ExprType::Paren)],
        "Error in function call, could be internal error",
    )?;
    let children = AST_Node::arc_mutex_get_children(node.clone());

    let function_input = eval_expr(children[0].clone())?;

    // function input must be tuple
    let function_input = function_input.to_tuple();

    let lexeme = AST_Node::get_token_lexeme_arc_mutex(node.clone());
    // let function: &LoxVariable;
    // stack_get!(function, &lexeme, node);
    let function = stack_get_variable(&lexeme, node)?;
    let function = function.lock().unwrap();

    match function.get_type() {
        LoxVariableType::STD_FUNCTION(_) => {
            return function.run_std_function(&function_input);
        }
        LoxVariableType::LOX_FUNCTION(_) => {
            return function.run_lox_function(&function_input);
        }
        _ => {}
    }
    Ok(LoxVariable::empty())
}

fn eval_expr_paren(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    // By parsing rule all expr(paren) will have at most one child, and the
    // child shall be expression
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() == 0 {
        // TODO: IT THIS THE PROPER WAY?
        // If there is nothing to return, shall we return LoxVariable NONE?
        return Ok(LoxVariable::empty_from_arc_mutex_ast_node(node.clone()));
    } else if children.len() == 1 {
        let a = eval_expr(children[0].clone())?;
        //     // DEBUG: line
        // match &a {
        //     Ok(o) => {
        //         // println!("{o}");
        //     }wh
        //     Err(e) => {}
        // }
        return Ok(a);
    } else {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            &format!(
                "Expr(Paren) has more than one {} children; likely a parsing error",
                children.len()
            ),
        ));
    }
}

fn eval_expr_negated(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    if !AST_Node::is_arc_mutex_expr(node.clone()) {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expected Expr. eval_expr_negated called on non-expr, likely internal error",
        ));
    }
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() == 0 {
        return Ok(LoxVariable::empty_from_arc_mutex_ast_node(node.clone()));
    } else if children.len() == 1 {
        let a = eval_expr(children[0].clone());
        let a = a.unwrap();
        return lox_negate(&a);
    } else {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expr(Paren) has more than one children; likely a parsing error",
        ));
    }
}

fn eval_tuple(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    let mut tuple: Vec<Box<LoxVariable>> = Vec::new();
    let children = AST_Node::arc_mutex_get_children(node.clone());
    for i in children {
        let a = eval_expr(i.clone())?;
        tuple.push(Box::new(a));
    }
    return Ok(LoxVariable::new(
        None,
        LoxVariableType::TUPLE(tuple),
        Some(node.clone()),
    ));
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
        AST_Type::Expr(ExprType::Negated) => {
            return eval_expr_negated(node.clone());
        }
        AST_Type::Identifier => {
            let lexeme = AST_Node::get_token_lexeme_arc_mutex(node.clone());
            // let variable: &LoxVariable;
            // stack_get!(variable, &lexeme, node);
            let variable = stack_get_variable(&lexeme, node)?;
            let variable = variable.lock().unwrap();
            return Ok(variable.clone());
        }
        AST_Type::Tuple => {
            return eval_tuple(node.clone());
        }
        _ => {
            debug!("{:?}", node);
            return Err(ErrorLox::from_arc_mutex_ast_node(
                node.clone(),
                "eval_expr called on non-expr, likely internal error",
            ));
        }
    }
}

fn exec_assignment(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() != 2 {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expected 2 children, likely a parsing error",
        ));
    }

    let left = children[0].clone();
    let right = children[1].clone();

    let lexeme = AST_Node::get_token_lexeme_arc_mutex(left.clone());
    let right = eval_expr(right.clone())?;

    let variable = stack_get_variable(&lexeme, left.clone())?;
    let mut variable = variable.lock().unwrap();
    variable.set_type(right.get_type());
    variable.set_ref_node(node.clone());

    Ok(variable.clone())
}

fn exec_something_equal(
    node: Arc<Mutex<AST_Node>>,
    lox_fun: fn(&LoxVariable, &LoxVariable) -> Result<LoxVariable, ErrorLox>,
) -> Result<LoxVariable, ErrorLox> {
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() != 2 {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expected 2 children, likely a parsing error",
        ));
    }

    let left = children[0].clone();
    let right = children[1].clone();

    let lexeme = AST_Node::get_token_lexeme_arc_mutex(left.clone());
    let right = eval_expr(right.clone())?;

    let variable = stack_get_variable(&lexeme, left.clone())?;
    let mut variable = variable.lock().unwrap();

    let res = lox_fun(&variable, &right)?;
    variable.set_type(res.get_type());

    Ok(variable.clone())
}

fn exec_plus_equal(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    return exec_something_equal(node, lox_add);
}

fn exec_minus_equal(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    return exec_something_equal(node, lox_minus);
}

fn exec_star_equal(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    return exec_something_equal(node, lox_multiply);
}

fn exec_slash_equal(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    return exec_something_equal(node, lox_divide);
}

fn exec_percent_equal(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    return exec_something_equal(node, lox_modula);
}

// var a = 1 will be parsed as:
// |-(=    EQUAL 1:7)      AST_Type::Stmt(Declaration)
//    |-(=    EQUAL 1:7)      AST_Type::Stmt(Assignment)
//       |-(a    IDENTIFIER 1:5)      AST_Type::Identifier
//       |-(1    NUMBER 1:9)      AST_Type::Expr(Normal)
fn exec_declaration(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    // get the lexeme
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() != 1 {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expected 1 children for declaration, likely a parsing error",
        ));
    }
    let assignment = children[0].clone();
    let children = AST_Node::arc_mutex_get_children(assignment.clone());
    if children.len() != 2 {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expected 2 children for assignment, likely a parsing error",
        ));
    }
    let left = children[0].clone();
    let right = children[1].clone();
    let mut variable = eval_expr(right.clone())?;
    variable.set_ref_node(node.clone());
    let lexeme = AST_Node::get_token_lexeme_arc_mutex(left.clone());
    variable.set_identifier(lexeme);

    stack_push(variable.clone());
    Ok(LoxVariable::empty())
}

fn exec_braced_stmt(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    stack::stack_new_scope();
    let res = execute_compound_stmt(node.clone());
    stack::stack_pop_scope();
    return res;
}

/// ```lox
/// if true {
///     print("true!");
/// }
/// ```
/// will be parsed into this
///(if   IF 1:1)      AST_Type::Stmt(If)
///  |-(true TRUE 1:4)      AST_Type::Expr(Normal)
///  |-({    LEFT_BRACE 1:9)      AST_Type::Stmt(Braced)
///     |-(\xa  STMT_SEP 3:0)      AST_Type::Stmt(Normal)
///        |-(print IDENTIFIER 2:2)      AST_Type::Expr(Function)
///           |-((    LEFT_PAREN 2:7)      AST_Type::Expr(Paren)
///              |-(true! STRING 2:8)      AST_Type::Expr(Normal)
///
/// for else if and else block, they will be parsed as these:
/// ```lox
///if true {
///	   var a = 1
///} else if true {
///	   var a = 2
///} else if true {
///	   var a = 2
///} else {
///	   1
///}
/// ```
///(if   IF 1:1)      AST_Type::Stmt(If)
///|-(true TRUE 1:4)      AST_Type::Expr(Normal)
///|-({    LEFT_BRACE 1:9)      AST_Type::Stmt(Braced)
///|  |-(=    EQUAL 2:8)      AST_Type::Stmt(Declaration)
///|     |-(=    EQUAL 2:8)      AST_Type::Stmt(Assignment)
///|        |-(a    IDENTIFIER 2:6)      AST_Type::Identifier
///|        |-(1    NUMBER 2:10)      AST_Type::Expr(Normal)
///|-(if   IF 3:8)      AST_Type::Stmt(Elseif)
///|  |-(true TRUE 3:11)      AST_Type::Expr(Normal)
///|  |-({    LEFT_BRACE 3:16)      AST_Type::Stmt(Braced)
///|     |-(=    EQUAL 4:8)      AST_Type::Stmt(Declaration)
///|        |-(=    EQUAL 4:8)      AST_Type::Stmt(Assignment)
///|           |-(a    IDENTIFIER 4:6)      AST_Type::Identifier
///|           |-(2    NUMBER 4:10)      AST_Type::Expr(Normal)
///|-(if   IF 7:2)      AST_Type::Stmt(Elseif)
///|  |-(true TRUE 7:5)      AST_Type::Expr(Normal)
///|  |-({    LEFT_BRACE 7:10)      AST_Type::Stmt(Braced)
///|     |-(=    EQUAL 8:8)      AST_Type::Stmt(Declaration)
///|        |-(=    EQUAL 8:8)      AST_Type::Stmt(Assignment)
///|           |-(a    IDENTIFIER 8:6)      AST_Type::Identifier
///|           |-(2    NUMBER 8:10)      AST_Type::Expr(Normal)
///|-(else ELSE 9:3)      AST_Type::Unparsed(ELSE)
///  |-({    LEFT_BRACE 9:8)      AST_Type::Stmt(Braced)
///     |-(\xa  STMT_SEP 11:0)      AST_Type::Stmt(Normal)
///        |-(1    NUMBER 10:2)      AST_Type::Expr(Normal)
fn exec_if_stmt(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    // check if node fits the grammar rule of if and else if statement. If not, return error
    // If it passes the check, evaluate the condition of if and return Ok(condition).
    // To eval the brace, do this
    // let children = AST_Node::arc_mutex_get_children(node.clone());
    // if error_handle_if_stmt(node.clone())? {
    //     return exec_braced_stmt(children[1].clone());
    // }
    fn error_handle_if_stmt(node: Arc<Mutex<AST_Node>>) -> Result<bool, ErrorLox> {
        if !AST_Node::arc_belongs_to_AST_type(
            node.clone(),
            &vec![
                AST_Type::Stmt(StmtType::If),
                AST_Type::Stmt(StmtType::Elseif),
            ],
        ) {
            return Err(ErrorLox::from_arc_mutex_ast_node(
                node.clone(),
                "Expected If or else if statement. Likely internal parsing or runtime error",
            ));
        }

        let children = AST_Node::arc_mutex_get_children(node.clone());
        if children.len() < 2 {
            return Err(ErrorLox::from_arc_mutex_ast_node(
                node.clone(),
                "If statement requires a condition and braced statement. Only fonnd one",
            ));
        }
        if !AST_Node::is_arc_mutex_expr(children[0].clone()) {
            return Err(ErrorLox::from_arc_mutex_ast_node(
                node.clone(),
                "Expected boolean expression after if",
            ));
        }

        // error check and eval the condition
        let condition = eval_expr(children[0].clone())?;
        if !condition.is_bool() {
            return Err(ErrorLox::from_lox_variable(
                &condition,
                "Expected boolean expression after if",
            ));
        }

        AST_Node::error_handle_check_type_arc(
            children[1].clone(),
            AST_Type::Stmt(StmtType::Braced),
            "Expected braced stmt after if",
        )?;
        Ok(condition.get_bool())
    }

    let children = AST_Node::arc_mutex_get_children(node.clone());
    if error_handle_if_stmt(node.clone())? {
        return exec_braced_stmt(children[1].clone());
    } else {
        for index in 2..children.len() {
            match AST_Node::get_AST_Type_from_arc(children[index].clone()) {
                AST_Type::Stmt(StmtType::Else) => {
                    AST_Node::error_handle_check_children_num_and_type_arc(
                        children[index].clone(),
                        &vec![AST_Type::Stmt(StmtType::Braced)],
                        "",
                    )?;
                    let else_children = AST_Node::arc_mutex_get_children(children[index].clone());
                    return exec_braced_stmt(else_children[0].clone());
                }
                AST_Type::Stmt(StmtType::Elseif) => {
                    let elseif_children = AST_Node::arc_mutex_get_children(children[index].clone());
                    if error_handle_if_stmt(children[index].clone())? {
                        return exec_braced_stmt(elseif_children[1].clone());
                    }
                }
                _ => {}
            }
        }
        return Ok(LoxVariable::empty());
    }
}

/// ```lox
/// while true {
///     var a = 1
/// }
/// ```
/// will be parsed into these
///(while WHILE 1:1)      AST_Type::Stmt(While)
/// |-(true TRUE 1:7)      AST_Type::Expr(Normal)
/// |-({    LEFT_BRACE 1:12)      AST_Type::Stmt(Braced)
///    |-(=    EQUAL 2:8)      AST_Type::Stmt(Declaration)
///       |-(=    EQUAL 2:8)      AST_Type::Stmt(Assignment)
///          |-(a    IDENTIFIER 2:6)      AST_Type::Identifier
///          |-(1    NUMBER 2:10)      AST_Type::Expr(Normal)
fn exec_while_stmt(node: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    let children = AST_Node::arc_mutex_get_children(node.clone());
    if children.len() < 2 {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "While statement requires a condition and braced statement. Only fonnd one",
        ));
    }
    if !AST_Node::is_arc_mutex_expr(children[0].clone()) {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expected boolean expression after while",
        ));
    }

    // error check and eval the condition
    let mut condition = eval_expr(children[0].clone())?;
    if !condition.is_bool() {
        return Err(ErrorLox::from_lox_variable(
            &condition,
            "Expected boolean expression after while",
        ));
    }

    // check is the second expr braced
    if AST_Node::get_AST_Type_from_arc(children[1].clone()) != AST_Type::Stmt(StmtType::Braced) {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            node.clone(),
            "Expected braced stmt after while",
        ));
    }

    let mut res: LoxVariable = LoxVariable::empty();
    while condition.get_bool() {
        res = exec_braced_stmt(children[1].clone())?;
        condition = eval_expr(children[0].clone())?;
    }
    Ok(res)
}

/// ```lox
/// fn none() {}
/// ```
/// will be parsed to this
///(fn   FN 5:1)      AST_Type::Stmt(FunctionDef)
///  |-(none IDENTIFIER 5:4)      AST_Type::Identifier
///  |-((    LEFT_PAREN 5:8)      AST_Type::Expr(Paren)
///  |-({    LEFT_BRACE 5:11)      AST_Type::Stmt(Braced)
///
///
///
///```lox
/// fn hello (a,b){
///	a + b
/// }
///```
///will be parsed into this
///(fn   FN 5:1)      AST_Type::Stmt(FunctionDef)
///  |-(none IDENTIFIER 5:4)      AST_Type::Identifier
///  |-((    LEFT_PAREN 5:8)      AST_Type::Expr(Paren)
///  |-({    LEFT_BRACE 5:11)      AST_Type::Stmt(Braced)
pub fn exec_function_definition(tree: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    AST_Node::error_handle_check_children_num_and_type_arc(
        tree.clone(),
        &vec![
            AST_Type::Identifier,
            AST_Type::Expr(ExprType::Paren),
            AST_Type::Stmt(StmtType::Braced),
        ],
        "Correct function definition is fn IDENTIFER (a,b,c) {}",
    )?;
    let children = AST_Node::arc_mutex_get_children(tree.clone());
    // the first child is the identifer
    let identifier = AST_Node::get_token_lexeme_arc_mutex(children[0].clone());
    // the second child is expr(paren), holding a tuple or nothing
    let mut tuple: Arc<Mutex<AST_Node>>;
    let expr_children = AST_Node::arc_mutex_get_children(children[1].clone());
    if expr_children.len() == 0 {
        tuple = AST_Node::dummy_node(AST_Type::Tuple).into();
    } else if expr_children.len() == 1 {
        // our later eval of the function expect a ast_node tuple as function input
        // a, b will be parsed as a tuple,  but "a" will only be parsed as a variable.
        // so we manually make a into a tuple
        match AST_Node::get_AST_Type_from_arc(expr_children[0].clone()) {
            AST_Type::Tuple => {
                tuple = expr_children[0].clone();
            }
            _ => {
                let mut tmp = AST_Node::new_from_ref(
                    &AST_Type::Tuple,
                    &AST_Node::arc_mutex_get_token(expr_children[0].clone())
                        .lock()
                        .unwrap(),
                );
                tmp.append_child(expr_children[0].clone());
                tuple = Arc::new(Mutex::new(tmp));
            }
        }
    } else {
        return Err(ErrorLox::from_arc_mutex_ast_node(
            children[1].clone(),
            "Expected one tuple, found multiple nodes",
        ));
    }

    let lox_function = LoxFunction::from_ast(tuple, children[2].clone())?;

    let funciton = LoxVariable::new(
        Some(identifier),
        LoxVariableType::LOX_FUNCTION(lox_function),
        Some(tree.clone()),
    );

    stack::stack_push(funciton.clone());
    stack::display_stack();
    Ok(funciton)
}

pub(crate) fn run(tree: Arc<Mutex<AST_Node>>) -> Result<LoxVariable, ErrorLox> {
    match AST_Node::get_AST_Type_from_arc(tree.clone()) {
        AST_Type::Expr(ExprType::Normal)
        | AST_Type::Expr(ExprType::Paren)
        | AST_Type::Expr(ExprType::Negated)
        | AST_Type::Expr(ExprType::Function) => {
            return eval_expr(tree.clone());
        }
        AST_Type::Tuple => {
            return eval_tuple(tree.clone());
        }
        AST_Type::Stmt(StmtType::Compound) => {
            return execute_compound_stmt(tree.clone());
        }
        AST_Type::Stmt(StmtType::Normal) => {
            return execute_compound_stmt(tree.clone());
        }
        AST_Type::Stmt(StmtType::Assignment) => {
            return exec_assignment(tree.clone());
        }
        AST_Type::Stmt(StmtType::PlusEqual) => {
            return exec_plus_equal(tree.clone());
        }
        AST_Type::Stmt(StmtType::MinusEqual) => {
            return exec_minus_equal(tree.clone());
        }
        AST_Type::Stmt(StmtType::StarEqual) => {
            return exec_star_equal(tree.clone());
        }
        AST_Type::Stmt(StmtType::SlashEqual) => {
            return exec_slash_equal(tree.clone());
        }
        AST_Type::Stmt(StmtType::PercentEqual) => {
            return exec_percent_equal(tree.clone());
        }
        AST_Type::Stmt(StmtType::Declaration) => {
            return exec_declaration(tree.clone());
        }
        AST_Type::Stmt(StmtType::Braced) => {
            return exec_braced_stmt(tree.clone());
        }
        AST_Type::Stmt(StmtType::If) => {
            return exec_if_stmt(tree.clone());
        }
        AST_Type::Stmt(StmtType::While) => {
            return exec_while_stmt(tree.clone());
        }
        AST_Type::Stmt(StmtType::FunctionDef) => {
            return exec_function_definition(tree.clone());
        }
        res => {
            println!("Unexecuted: {:?}", res);
        }
    }
    Ok(LoxVariable::empty())
}
