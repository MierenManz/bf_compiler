use leb128::write;

use super::Section;
use crate::EncodingError;
use crate::ValType;

pub struct TypeSection {
    type_defs: Vec<(Vec<ValType>, Vec<ValType>)>,
}

impl TypeSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_type_definition(&mut self, params: Vec<ValType>, returns: Vec<ValType>) -> usize {
        self.type_defs.push((params, returns));

        self.type_defs.len() - 1
    }

    pub fn remove_type_definition(&mut self, id: usize) -> bool {
        if id < self.type_defs.len() {
            self.type_defs.swap_remove(id);
        }

        id < self.type_defs.len()
    }
}

impl Default for TypeSection {
    fn default() -> Self {
        Self {
            type_defs: Vec::new(),
        }
    }
}

impl Section for TypeSection {
    fn compile(self) -> Result<Vec<u8>, EncodingError> {
        let mut byte_buff = Vec::with_capacity(5);
        write::unsigned(&mut byte_buff, self.type_defs.len() as u64)?;

        for (param_vector, result_vector) in self.type_defs {
            write::signed(&mut byte_buff, -0x20)?;
            write::unsigned(&mut byte_buff, param_vector.len() as u64)?;
            for x in param_vector {
                write::unsigned(&mut byte_buff, x as u64)?;
            }

            write::unsigned(&mut byte_buff, result_vector.len() as u64)?;
            for x in result_vector {
                write::unsigned(&mut byte_buff, x as u64)?;
            }
        }

        Ok(byte_buff)
    }

    fn id(&self) -> u8 {
        0x01
    }
}
