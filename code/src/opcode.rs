#![allow(warnings, unused)]
use lazy_static::*;
use AddrMode::*;

#[derive(Debug)]
pub struct Opcode {
    code: &'static str,
    hex: u8,
    len: u8,
    time : u8,
    mode: AddrMode,
}

impl Opcode {
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
enum AddrMode {
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
    static ref Opcode_Tab: Vec<Opcode> = vec![
    //BRK
        Opcode::new("BRK", 0x00, 1,7, Implied),

    //LDA
        Opcode::new("LDA", 0xa9, 2,3, Immediate),
        Opcode::new("LDA", 0xa5, 2,3, ZeroPage),
        Opcode::new("LDA", 0xb5, 2,4, ZeroPage_X),
        Opcode::new("LDA", 0xad, 3,4, Absolute),
        Opcode::new("LDA", 0xbd, 3,4, Absolute_X),
        Opcode::new("LDA", 0xb9, 3,4, Absolute_Y),
        Opcode::new("LDA", 0xa1, 2,6, Indirect_X),
        Opcode::new("LDA", 0xb1, 2,5, Indirect_Y),

    

    ];
}