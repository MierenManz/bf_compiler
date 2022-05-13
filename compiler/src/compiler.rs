use bf_common::Token;
use wasm_encoder::BlockType;
use wasm_encoder::CodeSection;
use wasm_encoder::Function;
use wasm_encoder::Instruction;
use wasm_encoder::MemArg;
use wasm_encoder::Module;
use wasm_encoder::ValType;

use crate::sections::*;

pub struct Compiler {
  module: Module,
  code_section: CodeSection,
}

impl Compiler {
  pub fn new() -> Self {
    let compiler = Compiler::default();
    compiler
  }

  pub fn compile(mut self, ast: &[Token]) -> Vec<u8> {
    let mut function = Function::new([(1, ValType::I32)]);
    let mem_arg = MemArg {
      align: 0,
      offset: 0,
      memory_index: 0,
    };
    for v in ast {
      match *v {
        Token::LocalGet => {
          function.instruction(&Instruction::LocalGet(0));
        }
        Token::LocalSet => {
          function.instruction(&Instruction::LocalSet(0));
        }
        Token::LocalTee => {
          function.instruction(&Instruction::LocalTee(0));
        }
        Token::Block => {
          function.instruction(&Instruction::Block(BlockType::Empty));
        }
        Token::Loop => {
          function.instruction(&Instruction::Loop(BlockType::Empty));
        }
        Token::End => {
          function.instruction(&Instruction::End);
        }
        Token::BranchIf(scope) => {
          function.instruction(&Instruction::BrIf(scope));
        }
        Token::Branch(scope) => {
          function.instruction(&Instruction::Br(scope));
        }
        Token::Call(fun) => {
          function.instruction(&Instruction::Call(fun as u32));
        }
        Token::I32Load8 => {
          function.instruction(&Instruction::I32Load8_U(mem_arg));
        }
        Token::I32Store8 => {
          function.instruction(&Instruction::I32Store8(mem_arg));
        }
        Token::I32Const(v) => {
          function.instruction(&Instruction::I32Const(v));
        }
        Token::I32Add => {
          function.instruction(&Instruction::I32Add);
        }
        Token::I32Eqz => {
          function.instruction(&Instruction::I32Eqz);
        }
      }
    }
    function.instruction(&Instruction::End);
    self.code_section.function(&function);

    self.module.section(&gen_type_section());
    self.module.section(&gen_import_section());
    self.module.section(&gen_fn_section());
    self.module.section(&gen_mem_section());
    self.module.section(&self.code_section);

    self.module.finish()
  }
}

impl Default for Compiler {
  fn default() -> Self {
    Self {
      module: Module::new(),
      code_section: CodeSection::new(),
    }
  }
}
