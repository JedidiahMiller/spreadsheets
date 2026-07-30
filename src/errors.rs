#[derive(Debug, Clone)]
pub enum ErrorType {
    UserError,
    UnexpectedError,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub error_type: ErrorType,
    pub error_message: String,
}

#[derive(Debug, Clone)]
pub struct SourceCodeError {
    pub location: Vec<usize>,
    pub error_message: String,
}
