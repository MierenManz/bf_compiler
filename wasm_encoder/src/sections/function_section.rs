use crate::sections::Section;
use crate::EncodingError;
use leb128::write;

pub struct FunctionSection {
    fn_declarations: Vec<u32>,
}

impl FunctionSection {
    pub fn new() -> Self {
        Self {
            fn_declarations: Vec::new(),
        }
    }
    pub fn add_fn_declaration(&mut self, type_index: u32) -> usize {
        self.fn_declarations.push(type_index);

        self.fn_declarations.len() - 1
    }

    pub fn remove_fn_declaration(&mut self, id: usize) -> bool {
        if id < self.fn_declarations.len() {
            self.fn_declarations.swap_remove(id);
        }

        id < self.fn_declarations.len()
    }
}

impl Section for FunctionSection {
    fn compile(self) -> Result<Vec<u8>, EncodingError> {
        let mut buff = Vec::with_capacity(self.fn_declarations.len() * 2);
        write::unsigned(&mut buff, self.fn_declarations.len() as u64)?;

        for decl in self.fn_declarations {
            write::unsigned(&mut buff, decl as u64)?;
        }

        Ok(buff)
    }

    fn id(&self) -> u8 {
        0x03
    }
}
