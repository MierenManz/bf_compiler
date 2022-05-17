use bf_common::Instruction;

#[derive(Debug)]
pub enum IdentType {
    Instruction(Instruction),
    Scope(Scope),
}

#[derive(Debug)]
pub struct Scope {
    idents: Vec<IdentType>,
}

impl Scope {
    pub fn new() -> Self {
        Scope { idents: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Scope {
            idents: Vec::with_capacity(capacity),
        }
    }

    pub fn add_ident(&mut self, ident: IdentType) {
        self.idents.push(ident);
    }

    pub fn add_idents(&mut self, idents: &mut Vec<IdentType>) {
        self.idents.append(idents);
    }
}

impl Default for Scope {
    fn default() -> Self {
        Scope::new()
    }
}
