use super::_Error;

#[derive(Debug)]
pub enum TypeError {
    Mismatch, // TODO: Add Expected and Found fields
}

impl _Error for TypeError {}
