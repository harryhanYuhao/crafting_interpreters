use crate::runtime::variable::{LoxVariableType, LoxVariable};
use crate::runtime::lox_std::conversion;
use crate::err_lox::ErrorLox;

pub(crate) fn print(input: &LoxVariable) -> Result<LoxVariable, ErrorLox>{
    let string = conversion::to_string(input)?;
    match string.get_type() {
        LoxVariableType::STRING(a) => {
            println!("{a}");
        }
        _ => {
            // return ErrorLox::from
        }
    }
    Ok(LoxVariable::empty())
}

