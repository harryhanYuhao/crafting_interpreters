use crate::token;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
enum Pumo {
    STATEMENT,
    EXPRESSION,
}

pub trait Leaf {}

pub trait LeafExpressionEval {
    fn eval(&self) -> i64;
}

impl fmt::Debug for dyn LeafExpressionEval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a")
    }
}

#[derive(Debug)]
struct TerminalExpr {
    token: Rc<token::Token>,
    pumo: Pumo,
}

impl TerminalExpr {
    /// Creates a new [`TerminalExpr`].
    fn new(token: Rc<token::Token>) -> TerminalExpr {
        TerminalExpr {
            token: token.clone(),
            pumo: Pumo::EXPRESSION,
        }
    }
}
impl Leaf for TerminalExpr {}
impl LeafExpressionEval for TerminalExpr {
    // TODO: error handling
    fn eval(&self) -> i64 {
        self.token.lexeme.parse().unwrap()
    }
}

#[derive(Debug)]
struct AddNonTerminal {
    left: Rc<dyn LeafExpressionEval>,
    right: Rc<dyn LeafExpressionEval>,
    pumo: Pumo,
}

impl AddNonTerminal {
    fn new(left: Rc<dyn LeafExpressionEval>, right: Rc<dyn LeafExpressionEval>) -> AddNonTerminal {
        AddNonTerminal {
            left,
            right,
            pumo: Pumo::EXPRESSION,
        }
    }
}

impl LeafExpressionEval for AddNonTerminal {
    fn eval(&self) -> i64 {
        self.left.eval() + self.right.eval()
    }
}

impl Leaf for AddNonTerminal {}

pub fn parse_tree(tokens: &Vec<Rc<token::Token>>) -> Rc<dyn LeafExpressionEval> {
    let vec_len = tokens.len();

    // end of recursion
    if vec_len == 1 {
        return Rc::new(TerminalExpr::new(tokens[0].clone()));
    }
    for i in 0..vec_len {
        match tokens[i].token_type {
            token::TokenType::PLUS => {
                return Rc::new(AddNonTerminal::new(
                    parse_tree(&tokens[..i].to_vec()),
                    parse_tree(&tokens[i + 1..].to_vec()),
                ))
            }
            _ => {}
        }
    }
    Rc::new(TerminalExpr::new(tokens[0].clone()))
}
