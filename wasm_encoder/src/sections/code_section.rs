use super::Section;

pub struct CodeSection {}

impl CodeSection {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CodeSection {
    fn default() -> Self {
        Self {}
    }
}

impl Section for CodeSection {
    fn compile(self) -> Vec<u8> {
        Vec::new()
    }
    fn id() -> u8 {
        0x01
    }
}