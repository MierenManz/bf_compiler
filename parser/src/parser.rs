use bf_ast::IdentType;
use bf_ast::Scope;
use bf_common::Instruction;
use bf_common::KeyWord;
use bf_lexer::Lexer;

use crate::error::ParseError;

pub struct Parser {
    keywords: Vec<KeyWord>,
    index: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            keywords: lexer.collect(),
            index: 0,
        }
    }

    fn within_index(&self, i: usize) -> bool {
        i < self.keywords.len()
    }

    fn within(&self) -> bool {
        self.within_index(self.index)
    }

    fn peek_index(&self, i: usize) -> Option<KeyWord> {
        if self.within_index(i) {
            Some(self.keywords[i])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<KeyWord> {
        self.peek_index(self.index)
    }

    fn step(&mut self) {
        self.index += 1;
    }

    fn parse_io_call(&mut self, scope: &mut Scope, fn_id: u8) {
        scope.add_ident(IdentType::Instruction(Instruction::Call(fn_id)));
    }

    fn parse_loop(&mut self, scope: &mut Scope) -> Result<(), ParseError> {
        let mut new_scope = Scope::new();
        while self.within() {
            self.parse_step(&mut new_scope)?;
        }

        scope.add_ident(IdentType::Scope(new_scope));

        Ok(())
    }

    fn parse_ptr(&mut self, scope: &mut Scope, keyword: KeyWord) -> Result<(), ParseError> {
        let mut count = if keyword == KeyWord::PtrIncrease {
            1
        } else {
            -1
        };

        while self.within() {
            let peek_value = self.peek().ok_or(ParseError::UnexpectedEOF)?;

            if peek_value == KeyWord::PtrIncrease {
                count += 1;
            } else if peek_value == KeyWord::PtrDecrease {
                count -= 1;
            } else {
                break;
            }

            self.step();
        }

        scope.add_idents(&mut vec![
            IdentType::Instruction(Instruction::LocalGet),
            IdentType::Instruction(Instruction::I32Const(count)),
            IdentType::Instruction(Instruction::I32Add),
            IdentType::Instruction(Instruction::LocalSet),
        ]);

        Ok(())
    }

    fn parse_value(&mut self, scope: &mut Scope, keyword: KeyWord) -> Result<(), ParseError> {
        let mut count = if keyword == KeyWord::ValueIncrease {
            1
        } else {
            -1
        };

        while self.within() {
            let peek_value = self.peek().ok_or(ParseError::UnexpectedEOF)?;

            if peek_value == KeyWord::ValueIncrease {
                count += 1;
            } else if peek_value == KeyWord::ValueDecrease {
                count -= 1;
            } else {
                break;
            }

            self.step();
        }

        scope.add_idents(&mut vec![
            IdentType::Instruction(Instruction::LocalGet),
            IdentType::Instruction(Instruction::LocalGet),
            IdentType::Instruction(Instruction::I32Load8),
            IdentType::Instruction(Instruction::I32Const(count)),
            IdentType::Instruction(Instruction::I32Add),
            IdentType::Instruction(Instruction::I32Store8),
        ]);

        Ok(())
    }

    fn parse_step(&mut self, mut scope: &mut Scope) -> Result<(), ParseError> {
        if self.within() {
            let keyword = self.peek().ok_or(ParseError::UnexpectedEOF)?;
            self.step();

            match keyword {
                KeyWord::IoRead => self.parse_io_call(&mut scope, 0),
                KeyWord::IoWrite => self.parse_io_call(&mut scope, 1),
                KeyWord::LoopStart => self.parse_loop(&mut scope)?,
                KeyWord::PtrIncrease | KeyWord::PtrDecrease => {
                    self.parse_ptr(&mut scope, keyword)?
                }
                KeyWord::ValueIncrease | KeyWord::ValueDecrease => {
                    self.parse_value(&mut scope, keyword)?
                }
                _ => return Ok(()),
            };
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<Scope, ParseError> {
        let mut scope = Scope::new();

        while self.within() {
            self.parse_step(&mut scope)?;
        }

        Ok(scope)
    }
}
