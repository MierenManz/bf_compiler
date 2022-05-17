#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    UnexpectedEOF,
    UnexpectedChar(char),
}
