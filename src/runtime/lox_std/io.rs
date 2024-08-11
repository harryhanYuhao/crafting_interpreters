use crate::err_lox::ErrorLox;
use crate::runtime::lox_std::conversion;
use crate::runtime::lox_variable::{LoxVariable, LoxVariableType};

pub(crate) fn check_function_input(input: &LoxVariable, length: usize) -> Result<(), ErrorLox> {
    if let Some(hint_len) = input.get_tuple_length() {
        if hint_len != length {
            return Err(ErrorLox::from_lox_variable(
                input,
                &format!("Expected {length} arguments, found {hint_len}"),
            ));
        }
    } else {
        // In such case input is not a tuple
        return Err(ErrorLox::from_lox_variable(
            input,
            &format!("Expected Tuple. Likely a parsing error"),
        ));
    }
    Ok(())
}

pub(crate) fn print_lox(input: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    check_function_input(input, 1)?;

    let content = input.get_tuple_content().unwrap();
    let string = conversion::lox_to_string(&content[0])?;
    match string.get_type() {
        LoxVariableType::STRING(a) => {
            println!("{a}");
        }
        _ => {
            return Err(ErrorLox::from_lox_variable(
                input,
                "Failed type conversion to String",
            ))
        }
    }
    Ok(LoxVariable::empty())
}

fn print(input: &LoxVariable) -> LoxVariable {
    match print_lox(input) {
        Err(e) => {
            e.panic();
        }
        _ => {}
    }
    LoxVariable::empty()
}

pub(crate) fn get_all() -> Vec<LoxVariable> {
    let mut ret = Vec::new();
    ret.push(LoxVariable::new(
        Some("print".to_string()),
        LoxVariableType::STD_FUNCTION(print),
        None,
    ));
    ret
}
