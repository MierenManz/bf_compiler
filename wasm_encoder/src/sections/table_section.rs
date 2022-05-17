use crate::sections::Section;
use crate::EncodingError;
use crate::ResizableLimits;
use leb128::write;

pub struct TableSection {
    tables: Vec<ResizableLimits>,
}

impl TableSection {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }
    pub fn add_table_descriptor(&mut self, descriptor: ResizableLimits) -> usize {
        self.tables.push(descriptor);

        self.tables.len() - 1
    }

    pub fn remove_table_descriptor(&mut self, id: usize) -> bool {
        if id < self.tables.len() {
            self.tables.swap_remove(id);
        }

        id < self.tables.len()
    }
}

impl Section for TableSection {
    fn compile(self) -> Result<Vec<u8>, EncodingError> {
        let mut buff = Vec::new();
        write::unsigned(&mut buff, self.tables.len() as u64)?;
        Ok(buff)
    }

    fn id(&self) -> u8 {
        0x04
    }
}
