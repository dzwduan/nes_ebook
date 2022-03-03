use crate::cpu::AddressingMode;
use crate::cpu::AddressingMode::*;
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    //static 存在于整个程序的生命周期
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode
}


impl OpCode {
    fn new(code:u8, mnemonic:&'static str,len:u8,cycles:u8, mode:AddressingMode) -> Self {
        OpCode {
            code,
            mnemonic,
            len,
            cycles,
            mode
        }
    }
}


lazy_static! {
    pub static ref CPU_OPS_TAB: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, NoneAddressing),
        OpCode::new(0xaa, "TAX", 1, 2, NoneAddressing),
        OpCode::new(0xe8, "INX", 1, 2, NoneAddressing),


        //LDA  bd b9 b1 +1 if page crosed
        OpCode::new(0xa9, "LDA", 2, 2, Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, Absolute),
        OpCode::new(0xbd, "LDA", 3, 3, Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4, Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 5, Indirect_Y),

        //STA
        OpCode::new(0x85, "STA", 2, 3, ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, ZeroPage_X),
        OpCode::new(0x8d, "STA", 3, 4, Absolute),
        OpCode::new(0x9d, "STA", 3, 5, Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, Indirect_Y),
    ];
}