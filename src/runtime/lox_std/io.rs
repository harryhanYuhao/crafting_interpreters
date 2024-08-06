use crate::err_lox::ErrorLox;
use crate::runtime::lox_std::conversion;
use crate::runtime::lox_variable::{LoxVariable, LoxVariableType};

pub(crate) fn print_lox (input: &LoxVariable) -> Result<LoxVariable, ErrorLox> {
    let string = conversion::lox_to_string(input)?;
    match string.get_type() {
        LoxVariableType::STRING(a) => {
            println!("{a}");
        }
        _ => return Err(ErrorLox::from_lox_variable(input, "Failed type conversion to String")),
    }
    Ok(LoxVariable::empty())
}
