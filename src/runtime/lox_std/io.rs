use crate::err_lox::ErrorLox;
use crate::runtime::lox_std::conversion;
use crate::runtime::lox_variable::{LoxVariable, LoxVariableType};

pub(crate) fn check_function_input(input: &LoxVariable, length: usize) -> Result<(), ErrorLox> {
    if input.get_tuple_length() != Some(length){
        return Err(ErrorLox::from_lox_variable(
            input,
            "Expected Tuple. Likely a Parser Error",
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
