use super::sections::*;
use crate::EncodingError;

pub struct Module {
    bytes: Vec<u8>,
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_section<T: Section>(&mut self, section: T) -> Result<(), EncodingError> {
        self.bytes.append(&mut section.compile()?);

        Ok(())
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            bytes: vec![0x00, 0x61, 0x73, 0x6D, 0x00, 0x00, 0x00, 0x01],
        }
    }
}
