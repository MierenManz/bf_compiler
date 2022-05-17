use super::Section;
use crate::EncodingError;
use crate::ResizableLimits;
use crate::ValType;
use leb128::write;

#[derive(Copy, Clone)]
pub enum ImportKind {
    Function(u32),
    Table(ResizableLimits),
    Memory(ResizableLimits),
    Global(ValType, bool),
}

impl ImportKind {
    pub fn encode(self) -> Result<Vec<u8>, EncodingError> {
        let mut buff = Vec::new();
        write::unsigned(&mut buff, u8::from(self) as u64)?;
        match self {
            ImportKind::Function(idx) => {
                write::unsigned(&mut buff, idx as u64)?;
            }
            ImportKind::Table(mem_descriptor) | ImportKind::Memory(mem_descriptor) => {
                if mem_descriptor.maximum.is_some() {
                    write::unsigned(&mut buff, 1)?;
                } else {
                    write::unsigned(&mut buff, 0)?;
                }

                write::unsigned(&mut buff, mem_descriptor.minimum as u64)?;

                if mem_descriptor.maximum.is_some() {
                    write::unsigned(&mut buff, mem_descriptor.maximum.unwrap() as u64)?;
                }
            }
            ImportKind::Global(val, is_mut) => {
                write::unsigned(&mut buff, val as u64)?;
                if is_mut {
                    write::unsigned(&mut buff, 1)?;
                } else {
                    write::unsigned(&mut buff, 0)?;
                }
            }
        };

        Ok(buff)
    }
}

impl From<ImportKind> for u8 {
    fn from(kind: ImportKind) -> Self {
        match kind {
            ImportKind::Function(_) => 0x00,
            ImportKind::Table(_) => 0x01,
            ImportKind::Memory(_) => 0x02,
            ImportKind::Global(_, _) => 0x03,
        }
    }
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

            byte_buff.extend_from_slice(&kind.encode()?);
        }

        Ok(byte_buff)
    }

    fn id(&self) -> u8 {
        0x02
    }
}
