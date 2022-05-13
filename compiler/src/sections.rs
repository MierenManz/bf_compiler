use wasm_encoder::EntityType;
use wasm_encoder::FunctionSection;
use wasm_encoder::ImportSection;
use wasm_encoder::MemorySection;
use wasm_encoder::MemoryType;
use wasm_encoder::TypeSection;
use wasm_encoder::ValType;

#[inline(always)]
pub fn gen_type_section() -> TypeSection {
  let mut ts = TypeSection::new();

  ts.function(vec![], vec![]);
  ts.function(vec![ValType::I32], vec![]);
  ts.function(vec![], vec![ValType::I32]);

  ts
}

#[inline(always)]
pub fn gen_import_section() -> ImportSection {
  let mut is = ImportSection::new();

  is.import("io", "read", EntityType::Function(2));
  is.import("io", "write", EntityType::Function(1));

  is
}

#[inline(always)]
pub fn gen_fn_section() -> FunctionSection {
  let mut fs = FunctionSection::new();
  fs.function(0);

  fs
}

#[inline(always)]
pub fn gen_mem_section() -> MemorySection {
  let mut ms = MemorySection::new();
  ms.memory(MemoryType {
    minimum: 1,
    maximum: None,
    memory64: false,
  });

  ms
}
