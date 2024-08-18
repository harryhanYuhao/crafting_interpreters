use crate::err_lox::ErrorLox;
use crate::runtime::lox_std::conversion;
use crate::runtime::lox_variable::{LoxVariable, LoxVariableType};

static PI: f64 = 3.141592653589793;

pub(crate) fn get_all() -> Vec<LoxVariable> {
    let mut ret = Vec::new();
    ret.push(LoxVariable::new(
        Some("PI".to_string()),
        LoxVariableType::NUMBER(PI),
        None,
    ));
    ret
}
