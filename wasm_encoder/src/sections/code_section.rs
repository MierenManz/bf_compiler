use crate::sections::Section;
use crate::EncodingError;

pub struct CodeSection {}

impl CodeSection {
    pub fn new() -> Self {
        Self {}
    }
}

impl Section for CodeSection {
    fn compile(self) -> Result<Vec<u8>, EncodingError> {
        Ok(Vec::new())
    }
    fn id(&self) -> u8 {
        0x01
    }
}
