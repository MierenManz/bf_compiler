use crate::sections::Section;
use crate::EncodingError;

pub struct MemorySection {}

impl MemorySection {
    pub fn new() -> Self {
        Self {}
    }
}

impl Section for MemorySection {
    fn compile(self) -> Result<Vec<u8>, EncodingError> {
        Ok(Vec::new())
    }
    fn id(&self) -> u8 {
        0x01
    }
}
