#[derive(Debug, Clone)]
pub struct Error {
    pos: (usize, usize),
    msg: String,
    //val: ScannerErrorType,
}

impl Error {
    pub fn new(pos: (usize, usize), msg: &str) -> Self {
        Self {
            pos,
            msg: msg.to_string(),
        }
    }
}
