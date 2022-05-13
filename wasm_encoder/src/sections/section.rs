use crate::EncodingError;

pub trait Section: Default {
    fn compile(self) -> Result<Vec<u8>, EncodingError>;
    fn id() -> u8;
}
