mod tokenizer;

use bf_common::KeyWord;
use tokenizer::Tokenizer;

pub struct Lexer {
    tokenizer: Tokenizer,
}

impl Lexer {
    pub fn new<T: Into<Vec<u8>>>(buf: T) -> Self {
        Self {
            tokenizer: Tokenizer::new(buf),
        }
    }
}

impl Iterator for Lexer {
    type Item = KeyWord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokenizer.within() {
            let byte = self.tokenizer.peek().unwrap();
            self.tokenizer.step();

            let keyword = match byte as char {
                '+' => KeyWord::ValueIncrease,
                '-' => KeyWord::ValueDecrease,
                '>' => KeyWord::PtrIncrease,
                '<' => KeyWord::PtrDecrease,
                '.' => KeyWord::IoWrite,
                ',' => KeyWord::IoRead,
                '[' => KeyWord::LoopStart,
                ']' => KeyWord::LoopEnd,
                _ => return self.next(),
            };

            Some(keyword)
        } else {
            None
        }
    }
}
