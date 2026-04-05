use super::_Error;

#[derive(Debug)]
pub enum TypeError {
    Mismatch,
}

impl _Error for TypeError {}
