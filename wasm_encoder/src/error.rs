use std::io::ErrorKind;

#[derive(Debug, Copy, Clone)]
pub enum EncodingError {
    CouldNotEncodeVarint,
    InvalidSectionID,
    CouldNotGetThread,
    TaskCancelled,
    TaskPanicked,
    MissingSection(&'static str),
}

impl From<std::io::Error> for EncodingError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            ErrorKind::Unsupported => Self::CouldNotGetThread,
            _ => Self::CouldNotEncodeVarint,
        }
    }
}
