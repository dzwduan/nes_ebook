
#![allow(warnings, unused)]

#[derive(Debug)]
pub struct CPU {
    pc: u16,
    sp: u8,
    reg_x: u8,
    reg_y: u8,
    reg_a: u8,
    status: Status
}

#[derive(Debug)]
struct Status {
    N: bool,
    O: bool,
    B: bool,
    D: bool,
    I: bool,
    Z: bool,
    C: bool
}

trait Mem {
    fn read_mem(addr: u16) -> u8;
    fn write_mem(addr:u16, data:u8);

    //为小端实现
    fn read_mem_u16(addr:u16) -> u8 {
        0
    }

    fn write_mem_u16(addr:u16, data:u8) {

    } 
}


impl Mem for CPU {

    fn read_mem(addr: u16) -> u8 {
        0
    }
    fn write_mem(addr:u16, data:u8) {

    }
}


impl CPU {

    fn new() -> Self {
        
    }
}