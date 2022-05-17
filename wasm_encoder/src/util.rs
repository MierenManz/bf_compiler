use crate::EncodingError;
use leb128::write;

#[derive(Copy, Clone)]
pub struct ResizableLimits {
    pub minimum: u32,
    pub maximum: Option<u32>,
}

#[derive(Copy, Clone)]
pub enum ValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
    FuncRef = 0x70,
    Func = 0x60,
    Void = 0x40,
}

#[derive(Copy, Clone)]
pub enum ExternalKind {
    Function(u32),
    Table(ResizableLimits),
    Memory(ResizableLimits),
    Global(ValType, bool),
}

impl ExternalKind {
    pub fn encode(self) -> Result<Vec<u8>, EncodingError> {
        let mut buff = Vec::new();
        write::unsigned(&mut buff, u8::from(self) as u64)?;
        match self {
            ExternalKind::Function(idx) => {
                write::unsigned(&mut buff, idx as u64)?;
            }
            ExternalKind::Table(mem_descriptor) | ExternalKind::Memory(mem_descriptor) => {
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
            ExternalKind::Global(val, is_mut) => {
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

impl From<ExternalKind> for u8 {
    fn from(kind: ExternalKind) -> Self {
        match kind {
            ExternalKind::Function(_) => 0x00,
            ExternalKind::Table(_) => 0x01,
            ExternalKind::Memory(_) => 0x02,
            ExternalKind::Global(_, _) => 0x03,
        }
    }
}
