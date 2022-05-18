pub struct Tokenizer {
    source: Vec<u8>,
    index: usize,
}

impl Tokenizer {
    pub fn new<T: Into<Vec<u8>>>(source: T) -> Self {
        Self {
            source: source.into(),
            index: 0,
        }
    }

    pub fn within_index(&self, i: usize) -> bool {
        i < self.source.len()
    }

    pub fn within(&self) -> bool {
        self.within_index(self.index)
    }

    pub fn peek_index(&self, i: usize) -> Option<u8> {
        if self.within_index(i) {
            return Some(self.source[i]);
        }

        None
    }

    pub fn peek(&self) -> Option<u8> {
        self.peek_index(self.index)
    }

    pub fn step(&mut self) {
        self.index += 1;
    }
}
