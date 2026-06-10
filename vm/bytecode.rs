#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    PushConst = 0,
    Pop = 1,
    LoadLocal = 2,
    StoreLocal = 3,
    LoadGlobal = 4,
    StoreGlobal = 5,
    Add = 6,
    Sub = 7,
    Mul = 8,
    Div = 9,
    Mod = 10,
    Eq = 11,
    Neq = 12,
    Lt = 13,
    Gt = 14,
    Le = 15,
    Ge = 16,
    And = 17,
    Or = 18,
    Not = 19,
    Jmp = 20,
    JmpIfFalse = 21,
    Call = 22,
    Ret = 23,
    NewList = 24,
    NewMap = 25,
    GetIndex = 26,
    SetIndex = 27,
    SysCall = 28,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        unsafe { std::mem::transmute(v) }
    }
}
