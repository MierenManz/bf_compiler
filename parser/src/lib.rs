use bf_common::CallableFunction;
use bf_common::KeyWord;
use bf_common::Token;
use bf_lexer::Lexer;

pub struct Parser {
    keywords: Vec<KeyWord>,
    index: usize,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let keywords = lexer.collect();
        Self { keywords, index: 0 }
    }
    pub fn within_index(&self, i: usize) -> bool {
        if i < self.keywords.len() {
            return true;
        }
        false
    }

    pub fn within_offset(&self, offset: usize) -> bool {
        self.within_index(self.index + offset)
    }

    pub fn within(&self) -> bool {
        self.within_index(self.index)
    }

    pub fn peek_index(&self, i: usize) -> Option<KeyWord> {
        if self.within_index(i) {
            return Some(self.keywords[i]);
        }
        None
    }

    pub fn peek_offset(&self, offset: isize) -> Option<KeyWord> {
        self.peek_index((self.index as isize + offset) as usize)
    }

    pub fn peek(&self) -> Option<KeyWord> {
        self.peek_index(self.index)
    }

    pub fn step(&mut self) {
        self.index += 1;
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut ast = Vec::with_capacity(self.keywords.len());
        let mut depth: i32 = i32::MAX;
        while self.within() {
            let word = self.peek().unwrap();
            self.step();
            match word {
                KeyWord::IoRead => ast.append(&mut vec![
                    Token::LocalGet,
                    Token::Call(CallableFunction::IoRead),
                    Token::I32Store8,
                ]),
                KeyWord::IoWrite => ast.append(&mut vec![
                    Token::LocalGet,
                    Token::I32Load8,
                    Token::Call(CallableFunction::IoWrite),
                ]),
                KeyWord::PtrIncrease | KeyWord::PtrDecrease => {
                    let mut count: i32 = if word == KeyWord::PtrIncrease { 1 } else { -1 };

                    while self.within() {
                        let v = self.peek().unwrap();

                        if v == KeyWord::PtrIncrease {
                            count += 1;
                        } else if v == KeyWord::PtrDecrease {
                            count -= 1;
                        } else {
                            break;
                        }
                        // Only step if keyword is increase or decrease
                        self.step();
                    }
                    if count != 0 {
                        ast.append(&mut vec![
                            Token::LocalGet,
                            Token::I32Const(count),
                            Token::I32Add,
                            Token::LocalSet,
                        ]);
                    }
                }
                KeyWord::ValueDecrease | KeyWord::ValueIncrease => {
                    let mut count: i32 = if word == KeyWord::ValueIncrease {
                        1
                    } else {
                        -1
                    };

                    while self.within() {
                        let v = self.peek().unwrap();

                        if v == KeyWord::ValueIncrease {
                            count += 1;
                        } else if v == KeyWord::ValueDecrease {
                            count -= 1;
                        } else {
                            break;
                        }
                        // Only step if keyword is increase or decrease
                        self.step();
                    }
                    if count != 0 {
                        ast.append(&mut vec![
                            Token::LocalGet,
                            Token::LocalGet,
                            Token::I32Load8,
                            Token::I32Const(count),
                            Token::I32Add,
                            Token::I32Store8,
                        ]);
                    }
                }
                KeyWord::LoopStart => {
                    // `[-]` loop
                    if self
                        .peek_offset(1)
                        .and_then(|x| Some(x == KeyWord::ValueDecrease))
                        .and(Some(false))
                        .unwrap()
                        && self
                            .peek_offset(2)
                            .and_then(|x| Some(x == KeyWord::LoopEnd))
                            .and(Some(false))
                            .unwrap()
                    {
                        self.step();
                        self.step();
                        ast.append(&mut vec![
                            Token::LocalGet,
                            Token::I32Const(0),
                            Token::I32Store8,
                        ])
                    } else {
                        depth += 1;
                        let calc_depth = depth - i32::MAX;
                        ast.append(&mut vec![
                            Token::Block,
                            Token::Loop,
                            Token::LocalGet,
                            Token::I32Load8,
                            Token::I32Eqz,
                            Token::BranchIf((calc_depth * calc_depth) as u32),
                        ]);
                    }
                }
                KeyWord::LoopEnd => {
                    depth -= 1;
                    let calc_depth = depth - i32::MAX;
                    ast.append(&mut vec![
                        Token::Branch((calc_depth * calc_depth) as u32),
                        Token::End,
                    ]);
                }
            };

            // dbg!(word, len, ast.len(), &ast[len..ast.len()]);
        }

        ast
    }
}
