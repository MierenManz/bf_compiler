use super::Section;
use crate::EncodingError;
use leb128::write;

pub enum ImportKind {
    Function = 0x00,
    Table = 0x1,
    Memory = 0x02,
    Global = 0x03,
}

pub struct ImportSection {
    imports: Vec<(String, String, ImportKind)>,
}

impl ImportSection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_import<T: Into<String>>(
        &mut self,
        module_name: T,
        export_name: T,
        kind: ImportKind,
    ) -> usize {
        self.imports
            .push((module_name.into(), export_name.into(), kind));

        self.imports.len() - 1
    }

    pub fn remove_import(&mut self, id: usize) -> bool {
        if id < self.imports.len() {
            self.imports.swap_remove(id);
            true
        } else {
            false
        }
    }
}

impl Default for ImportSection {
    fn default() -> Self {
        Self {
            imports: Vec::new(),
        }
    }
}

impl Section for ImportSection {
    fn compile(self) -> Result<Vec<u8>, EncodingError> {
        let mut byte_buff = Vec::new();
        write::unsigned(&mut byte_buff, self.imports.len() as u64)?;

        for (module_name, export_name, kind) in self.imports {
            write::unsigned(&mut byte_buff, module_name.len() as u64)?;
            byte_buff.extend_from_slice(module_name.as_bytes());

            write::unsigned(&mut byte_buff, export_name.len() as u64)?;
            byte_buff.extend_from_slice(export_name.as_bytes());

            byte_buff.push(kind as u8);
        }

        Ok(byte_buff)
    }

    fn id(&self) -> u8 {
        0x02
    }
}
