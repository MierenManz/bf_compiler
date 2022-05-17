use super::Section;
use crate::EncodingError;
use crate::ExternalKind;
use leb128::write;

pub struct ImportSection {
    imports: Vec<(String, String, ExternalKind)>,
}

impl ImportSection {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }

    pub fn add_import<T: Into<String>>(
        &mut self,
        module_name: T,
        export_name: T,
        kind: ExternalKind,
    ) -> usize {
        self.imports
            .push((module_name.into(), export_name.into(), kind));

        self.imports.len() - 1
    }

    pub fn remove_import(&mut self, id: usize) -> bool {
        if id < self.imports.len() {
            self.imports.swap_remove(id);
        }

        id < self.imports.len()
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

            byte_buff.extend_from_slice(&kind.encode()?);
        }

        Ok(byte_buff)
    }

    fn id(&self) -> u8 {
        0x02
    }
}
