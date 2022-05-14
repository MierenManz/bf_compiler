use super::Section;
use crate::EncodingError;

pub enum ImportKind {
    Function = 0x00,
    Table = 0x1,
    Memory = 0x02,
    Global = 0x03
}

pub struct ImportSection {
    imports: Vec<(String, String, ImportKind)>
}

impl ImportSection {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }


    pub fn add_import<T: Into<String>>(&mut self, namespace: T, fn_name: T, kind: ImportKind) -> usize {
        self.imports.push((namespace.into(), fn_name.into(), kind));

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


impl Section for ImportSection {
    fn compile(self) -> Result<Vec<u8>, EncodingError> {
        Ok(Vec::new())
    }

    fn id() -> u8 {
        0x02
    }
}
