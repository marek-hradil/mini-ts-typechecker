#[derive(Debug, Clone)]
pub enum LexingError {
    UnterminatedStringLiteralError,
    UnexpectedEndOfFileError,
}
