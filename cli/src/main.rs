use bf_compiler::Compiler;
use bf_lexer::Lexer;
use bf_parser::Parser;
use std::fs::OpenOptions;
use std::io::Read;

pub fn main() {
  let mut bytes = Vec::new();

  {
    let mut file_handle = OpenOptions::new()
      .read(true)
      .open(&std::env::args().collect::<Vec<String>>()[1])
      .expect("Error at opening file");

    file_handle
      .read_to_end(&mut bytes)
      .expect("Error at reading file");
  }

  let wasm_bytes = Compiler::new().compile(&Parser::new(Lexer::new(bytes)).parse());

  std::fs::write("./out.wasm", wasm_bytes).unwrap();
}
