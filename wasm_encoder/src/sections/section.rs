use crate::EncodingError;

pub trait Section {
    fn compile(self) -> Result<Vec<u8>, EncodingError>;
    fn id(&self) -> u8;
}
