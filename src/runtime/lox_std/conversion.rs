use crate::err_lox::ErrorLox;
use crate::interpreter::AST_Node::AST_Node;
use crate::runtime::lox_variable::{LoxVariable, LoxVariableType};

fn to_string_runtime(variable: &LoxVariable) -> String {
    let mut string = String::new();
    match variable.get_type() {
        LoxVariableType::NONE => {}
        LoxVariableType::BOOL(a) => {
            string = format!("{a}");
        }
        LoxVariableType::NUMBER(a) => {
            string = format!("{a}");
        }
        LoxVariableType::STD_FUNCTION(a) => {
            string = format!("{a:?}");
        }
        LoxVariableType::STRING(s) => {
            string = s.clone();
        }
        LoxVariableType::TUPLE(t) => {
            let mut res = String::from("(");
            for i in t.iter() {
                let tmp = to_string_runtime(&i);
                res.push_str(&tmp);
                res.push_str(", ");
            }
            res.pop();
            res.pop();
            res.push(')');
            string = res;
        }
    }
    string
}

pub fn lox_to_string(variable: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    let string = to_string_runtime(variable);

    Ok(LoxVariable::new(
        None,
        LoxVariableType::STRING(string),
        variable.get_ref_node(),
    ))
}
