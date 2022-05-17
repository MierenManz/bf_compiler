use super::Section;

pub struct ExportSection {}

impl ExportSection {
    pub fn new() -> Self {
        Self {}
    }
}

impl Section for ExportSection {
    fn compile(self) -> Vec<u8> {
        Vec::new()
    }
    fn id() -> u8 {
        0x01
    }
}
