use super::sections::*;
use crate::EncodingError;

#[cfg(feature = "async_compile")]
use tokio::task;
#[cfg(feature = "async_compile")]
use tokio::task::JoinError;
#[cfg(feature = "async_compile")]
use tokio::task::JoinHandle;

pub struct Module {
    type_section: Vec<TypeSection>,
    import_section: Vec<ImportSection>,
    function_section: Vec<FunctionSection>,
    // table_section: Vec<TableSection>,
    // memory_section: Vec<MemorySection>,
    // export_section: Vec<ExportSection>,
    // code_section: Vec<CodeSection>,
}

impl Module {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_type_section(&mut self, section: TypeSection) -> usize {
        self.type_section.push(section);
        self.type_section.len() - 1
    }

    pub fn add_import_section(&mut self, section: ImportSection) -> usize {
        self.import_section.push(section);
        self.import_section.len() - 1
    }

    pub fn add_function_section(&mut self, section: FunctionSection) -> usize {
        self.function_section.push(section);
        self.function_section.len() - 1
    }

    // pub fn add_memory_section(&mut self, section: MemorySection) {}
    // pub fn add_export_section(&mut self, section: ExportSection) {}
    // pub fn add_code_section(&mut self, section: CodeSection) {}

    #[cfg(not(feature = "async_compile"))]
    pub fn compile(self) -> Result<Vec<u8>, EncodingError> {
        if self.type_section.len() == 0 {
            return Err(EncodingError::MissingSection("Type Section"));
        }

        if self.function_section.len() == 0 {
            return Err(EncodingError::MissingSection("Function Section"));
        }

        // if self.code_section.len() == 0 {
        //     return Err(EncodingError::MissingSection("Code Section"));
        // }

        let mut buffer = Vec::new();

        for s in self.type_section {
            buffer.extend_from_slice(&s.compile()?);
        }

        for s in self.import_section {
            buffer.extend_from_slice(&s.compile()?);
        }

        for s in self.function_section {
            buffer.extend_from_slice(&s.compile()?);
        }
        // for s in self.memory_section {
        // buffer.extend_from_slice(&s.compile()?);
        // }
        // for s in self.export_section {

        // buffer.extend_from_slice(&s.compile()?);
        // }
        // for s in self.code_section {
        // buffer.extend_from_slice(&s.compile()?);
        // }

        Ok(buffer)
    }

    #[cfg(feature = "async_compile")]
    pub async fn compile(self) -> Result<Vec<u8>, EncodingError> {
        let handle_count = self.import_section.len() + self.type_section.len();
        let mut handles = Vec::with_capacity(handle_count);

        for s in self.type_section {
            let handle = task::spawn(async { s.compile() });
            handles.push(handle);
        }

        for s in self.import_section {
            let handle = task::spawn(async { s.compile() });
            handles.push(handle);
        }

        for s in self.function_section {
            let handle = task::spawn(async { s.compile() });
            handles.push(handle);
        }
        // for s in self.memory_section {
        //     let handle = task::spawn(async { s.compile() });
        //     handles.push(handle);
        // }
        // for s in self.export_section {
        //     let handle = task::spawn(async { s.compile() });
        //     handles.push(handle);
        // }
        // for s in self.code_section {
        //     let handle = task::spawn(async { s.compile() });
        //     handles.push(handle)
        // }

        let mut buffer = Vec::new();

        for handle in handles {
            let v = handle.await.unwrap();
            buffer.extend_from_slice(&v?);
        }
        Ok(buffer)
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            type_section: Vec::new(),
            import_section: Vec::new(),
            function_section: Vec::new(),
            // table_section: Vec::new(),
            // memory_section: Vec<MemorySection>,
            // export_section: Vec<ExportSection>,
            // code_section: Vec<CodeSection>,
        }
    }
}
