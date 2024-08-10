/// STD functions
///
/// signature: fn(&LoxVariable) -> LoxVariable
/// Moreover, the input is expected to be a tuple
///
/// User defined functions behaves differently
pub mod conversion;
pub mod io;

use super::lox_variable::LoxVariable;

pub(crate) fn get_std() -> Vec<LoxVariable> {
    [crate::runtime::lox_std::io::get_all()].concat()
}
