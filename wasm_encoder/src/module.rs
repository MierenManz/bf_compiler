use super::sections::*;
use crate::EncodingError;

pub struct Module {
    bytes: Vec<u8>,
    current_section: u8,
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    /// Footgun because this method does not look at the type of module
    pub fn add_section(&mut self, section: &impl Section) -> Result<(), EncodingError> {
        let section_id = section.id();

        if self.current_section > section_id {
            return Err(EncodingError::BadSectionID);
        }

        if self.current_section < section_id {
            self.current_section = section_id;
        }

        if section.id() > 10 {
            return Err(EncodingError::InvalidSectionID);
        }

        Ok(())
    }

    pub fn finalize_as_slice(&self) -> &[u8] {
        &self.bytes
    }

    pub fn finalize(self) -> Vec<u8> {
        self.bytes
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            bytes: vec![0x00, 0x61, 0x73, 0x6D, 0x00, 0x00, 0x00, 0x01],
            current_section: 0,
        }
    }
}
