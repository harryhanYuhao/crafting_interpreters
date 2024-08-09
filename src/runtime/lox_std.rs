pub mod conversion;
pub mod io;

use super::lox_variable::LoxVariable;

pub(crate) fn get_std() -> Vec<LoxVariable> {
    [crate::runtime::lox_std::io::get_all()].concat()
}
