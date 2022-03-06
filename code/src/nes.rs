
#![allow(warnings, unused)]
use core::slice::*;
use std::ops::Add;
use crate::opcode::AddrMode::{self, *};

#[derive(Debug)]
pub struct CPU {
    pc: u16,
    sp: u8,
    reg_x: u8,
    reg_y: u8,
    reg_a: u8,
    status: Status,
    mem: [u8; 0xffff],
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

impl Status {
    fn zero_init() -> Self {
        Self {
            N: false,
            O: false,
            B: false,
            D: false,
            I: false,
            Z: false,
            C: false

        }
    }
}


trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr:u16, data:u8);

    //为小端实现, 从小端地址读取数据，ad 00 80 读到的返回值是 0x8000, 00是低位
    fn mem_read_u16(&self, addr:u16) -> u16 {
        let lo = self.mem_read(addr);
        let hi = self.mem_read(addr + 1);

        (hi as u16) << 8 | (lo as u16) & 0xff
    }

    fn mem_write_u16(&mut self, addr:u16, data:u16) {
        let lo = (data & 0xff) as u8;
        let hi = ((data << 8) & 0xff) as u8;

        self.mem_write(addr, lo);
        self.mem_write(addr+1, hi);
    } 
}

//1. 数组问题, why u16 cannot [u8]
//2. 


impl Mem for CPU {

    fn mem_read(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
    fn mem_write(&mut self, addr:u16, data:u8) {
        self.mem[addr as usize] = data;
    }
}



impl CPU {

    fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            reg_x: 0,
            reg_y: 0,
            reg_a: 0,
            status: Status::zero_init(),
            mem: [0; 0xffff],
        }
    }


    //reset 方法恢复所有寄存器的状态，通过存储在0xFFFC处的两个字节初始化PC
    fn reset(&mut self) {
        self.reg_a = 0;
        self.reg_x = 0;
        self.reg_y = 0;
        self.pc = self.mem_read_u16(0xfffc);
        self.status = Status::zero_init();
    }
    
    //将程序加载到ROM空间0x8000并且在0xfffc处初始化PC
    fn load(&mut self, program: Vec<u8>) {
        self.mem[0x8000 as usize..(0x8000 + program.len()) as usize].copy_from_slice(&program[..]);
        self.mem_write_u16(0xfffc, 0x8000);
    }

    fn run(&mut self) {
        let pc = 
    }

    pub fn load_and_run(&self, program: Vec<u8>) {
        self.reset();
        self.load(program);
        self.run();
    }


    fn get_addr(&mut self, mode:AddrMode) -> u16 {
        match mode {
            Immediate    => self.pc,
            Absolute     => self.mem_read_u16(self.pc),
            Absolute_X   => self.mem_read_u16(self.pc) + self.reg_x as u16,
            Absolute_Y   => self.mem_read_u16(self.pc) + self.reg_y as u16,
            ZeroPage     => self.pc & 0xff,
            ZeroPage_X   => {
                let base = self.mem_read(self.pc);
                let res = base.wrapping_add(self.reg_x);
                res as u16
            },
            ZeroPage_Y   => {
                let base = self.mem_read(self.pc);
                let res = base.wrapping_add(self.reg_y);
                res as u16
            },
            Indirect     => {
                self.mem_read_u16(self.pc)
            }

            Indirect_X   => {
                let base = self.mem_read(self.pc);
                let lo = base.wrapping_add(self.reg_x);
                let hi = base.wrapping_add(1);

                (hi as u16)<<8 | (lo as u16) &0xff
            },

            Indirect_Y   => {
                let base = self.mem_read(self.pc);
                let lo =base.wrapping_add(self.reg_y);
                let hi = base.wrapping_add(1).wrapping_add(self.status.C as u8);
                
                (hi as u16)<<8 | (lo as u16) &0xff
            },

            //TODO:
            Relative     => self.pc + self.mem_read(self.pc) as u16,
            Implied      => panic!("implicit address, do nothint!"),
        }
    }
}