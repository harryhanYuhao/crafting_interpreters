use crate::err_lox::ErrorLox;
use crate::interpreter::AST_Node::AST_Node;
use crate::runtime::variable::{LoxVariable, LoxVariableType};

pub fn to_string(variable: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    let mut string = String::new();
    match variable.get_type() {
        LoxVariableType::NONE => {}
        LoxVariableType::BOOL(a) => {
            string = format!("{a}");
        }
        LoxVariableType::NUMBER(a) => {
            string = format!("{a}");
        }
        LoxVariableType::FUNCTION(a) => {
            string = format!("{a:?}");
        }
        LoxVariableType::STRING(s) => {
            string = s.clone();
        }
    }

    Ok(LoxVariable::new(
        None,
        LoxVariableType::STRING(string),
        variable.get_ref_node(),
    ))
}
