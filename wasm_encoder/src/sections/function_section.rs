use super::Section;

pub struct FunctionSection {}

impl FunctionSection {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for FunctionSection {
    fn default() -> Self {
        Self {}
    }
}

impl Section for FunctionSection {
    fn compile(self) -> Vec<u8> {
        Vec::new()
    }
    fn id() -> u8 {
        0x03
    }
}
