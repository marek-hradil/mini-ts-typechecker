#[derive(Debug, Clone)]
pub enum LexingError {
    UnterminatedStringLiteralError,
    UnexpectedEndOfFileError,
}

#[derive(Debug, Clone)]
pub enum ParsingError {
    UnexpectedEndOfFileError,
}

pub enum BindingError {
    CannotRedeclareError,
}
