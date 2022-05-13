use super::Section;

pub struct MemorySection {}

impl MemorySection {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MemorySection {
    fn default() -> Self {
        Self {}
    }
}

impl Section for MemorySection {
    fn compile(self) -> Vec<u8> {
        Vec::new()
    }
    fn id() -> u8 {
        0x01
    }
}