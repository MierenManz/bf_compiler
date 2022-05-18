use crate::sections::Section;
use crate::EncodingError;

pub struct ExportSection {}

impl ExportSection {
    pub fn new() -> Self {
        Self {}
    }
}

impl Section for ExportSection {
    fn compile(self) -> Result<Vec<u8>, EncodingError> {
        Ok(Vec::new())
    }
    fn id(&self) -> u8 {
        0x01
    }
}
