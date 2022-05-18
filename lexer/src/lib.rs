mod tokenizer;
use bf_common::KeyWord;
use tokenizer::Tokenizer;

pub struct Lexer {
    buffer_size: usize,
    tokenizer: Tokenizer,
}

impl Lexer {
    pub fn new<T: Into<Vec<u8>>>(buf: T) -> Self {
        let buffer = buf.into();
        Self {
            buffer_size: buffer.len(),
            tokenizer: Tokenizer::new(buffer),
        }
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

impl Iterator for Lexer {
    type Item = KeyWord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tokenizer.within() {
            let byte = self.tokenizer.peek()?;
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

            return Some(keyword);
        }

        None
    }
}
