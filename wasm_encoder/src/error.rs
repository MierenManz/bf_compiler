#[derive(Debug, Copy, Clone)]
pub enum EncodingError {
    CouldNotEncodeVarint,
}

impl From<std::io::Error> for EncodingError {
    fn from(_: std::io::Error) -> Self {
        Self::CouldNotEncodeVarint
    }
}