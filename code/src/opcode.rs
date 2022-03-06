#![allow(warnings, unused)]
use std::collections::HashMap;

use lazy_static::*;
use AddrMode::*;

#[derive(Debug)]
pub struct OpCode {
    pub code: &'static str,
    pub hex: u8,
    pub len: u8,
    pub time : u8,
    pub mode: AddrMode,
}

impl OpCode {
    fn new(code:&'static str, hex:u8, len:u8, time:u8, mode:AddrMode) -> Self {
        Self {
            code,
            hex,
            len,
            time,
            mode
        }
    }
}


//refer Emulator 101
#[derive(Debug)]
pub enum AddrMode {
    Absolute_X,
    Absolute_Y,
    Absolute,
    Immediate,
    Indirect,
    Indirect_X,
    Indirect_Y,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Implied,
    Relative,
}



lazy_static! {
    pub static ref OpCode_TAB: Vec<OpCode> = vec![
    //BRK
        OpCode::new("BRK", 0x00, 1, 7, Implied),
        OpCode::new("TAX", 0xaa, 1, 2, Implied),
        OpCode::new("INX", 0xe8, 1, 2, Implied),

    //LDA
        OpCode::new("LDA", 0xa9, 2, 3, Immediate),
        OpCode::new("LDA", 0xa5, 2, 3, ZeroPage),
        OpCode::new("LDA", 0xb5, 2, 4, ZeroPage_X),
        OpCode::new("LDA", 0xad, 3, 4, Absolute),
        OpCode::new("LDA", 0xbd, 3, 4, Absolute_X),
        OpCode::new("LDA", 0xb9, 3, 4, Absolute_Y),
        OpCode::new("LDA", 0xa1, 2, 6, Indirect_X),
        OpCode::new("LDA", 0xb1, 2, 5, Indirect_Y),

        OpCode::new("STA", 0x85, 2, 3, ZeroPage),
        OpCode::new("STA", 0x95, 2, 4, ZeroPage_X),
        OpCode::new("STA", 0x8d, 3, 4, Absolute),
        OpCode::new("STA", 0x9d, 3, 5, Absolute_X),
        OpCode::new("STA", 0x99, 3, 5, Absolute_Y),
        OpCode::new("STA", 0x81, 2, 6, Indirect_X),
        OpCode::new("STA", 0x91, 2, 6, Indirect_Y),
    ];


    //OpCode是全局存在的
    pub static ref OpCode_Hash: HashMap<u8, &'static OpCode> = {
        let mut kv = HashMap::new();

        for op in &*OpCode_TAB {
            kv.insert(op.hex, op);
        }
        kv
    };
}