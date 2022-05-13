#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeyWord {
    PtrIncrease,
    PtrDecrease,
    ValueIncrease,
    ValueDecrease,
    IoRead,
    IoWrite,
    LoopStart,
    LoopEnd,
}

#[derive(Clone, Copy, Debug)]
pub enum Token {
    LocalGet,
    LocalSet,
    LocalTee,

    Block,
    Loop,
    End,
    BranchIf(u32),
    Branch(u32),

    Call(CallableFunction),

    I32Load8,
    I32Store8,
    I32Const(i32),
    I32Add,
    I32Eqz,
}

#[derive(Clone, Copy, Debug)]
pub enum CallableFunction {
    IoRead = 0,
    IoWrite = 1,
}
