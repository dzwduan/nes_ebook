
#![allow(warnings, unused)]
use core::slice::*;
use std::{ops::Add, collections::HashMap};
use crate::opcode::AddrMode::{self, *};
use crate::opcode::OpCode;
use crate::opcode;

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
        let hi = ((data >> 8) & 0xff) as u8;

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
        //1. 识别指令
        //2. pc+1
        //3. 识别行为
        //4. update pc
        let ref opcodes: HashMap<u8, &'static OpCode> = *opcode::OpCode_Hash;

        loop {
            let hex = self.mem_read(self.pc);
            self.pc += 1;
            
            let op_struct =  opcodes.get(&hex).expect(&format!("\u{1B}[31munimplemented code 0x{:x}\u{1B}[0m", hex));

            let pc_old = self.pc;

            match hex {
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    self.lda(&op_struct.mode);
                    println!("exec lda");
                },
                 /* STA */
                 0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.sta(&op_struct.mode);
                    println!("exec sta");
                }
                
                0xAA => self.tax(),
                0xe8 => self.inx(),
                0x00 => return,
                _ => todo!(),
            }

            //TODO:
            self.pc += (op_struct.len - 1) as u16;
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }


    fn get_addr(&mut self, mode:&AddrMode) -> u16 {
        match mode {
            Immediate    => self.pc,
            Absolute     => self.mem_read_u16(self.pc),
            Absolute_X   => self.mem_read_u16(self.pc) + self.reg_x as u16,
            Absolute_Y   => self.mem_read_u16(self.pc) + self.reg_y as u16,
            ZeroPage     => self.mem_read(self.pc) as u16,
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
            Indirect     =>  self.mem_read_u16(self.pc),
          

            Indirect_X   => {
                let base = self.mem_read(self.pc);
                let ptr = base.wrapping_add(self.reg_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);

                (hi as u16)<<8 | (lo as u16) &0xff
            },

            Indirect_Y   => {
                let base = self.mem_read(self.pc);
                let ptr = base.wrapping_add(self.reg_y);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                let res_hex = (hi as u16)<<8 | (lo as u16) &0xff;
                res_hex.wrapping_add(self.reg_y as u16)
            },

            //TODO:
            Relative     => self.pc + self.mem_read(self.pc) as u16,
            Implied      => panic!("implicit address, do nothing!"),
        }
    }

    fn lda(&mut self, mode:&AddrMode) {
        let addr = self.get_addr(mode);
        self.reg_a = self.mem_read(addr);
        self.update_negative_flag(self.reg_a);
        self.update_zero_flag(self.reg_a);
    }

    fn sta(&mut self, mode:&AddrMode) {
        let addr = self.get_addr(mode);
        self.mem_write(addr, self.reg_a);
    }

    fn tax(&mut self) {
        self.reg_x = self.reg_a;
        self.update_negative_flag(self.reg_x);
        self.update_zero_flag(self.reg_x);
    }

    fn inx(&mut self) {
        self.reg_x = self.reg_x.wrapping_add(1);
        self.update_negative_flag(self.reg_x);
        self.update_zero_flag(self.reg_x);
    }

    fn update_zero_flag(&mut self, result: u8) {
        if result == 0 {
            self.status.Z = true;
        } else {
            self.status.Z = false;
        }
    }

    fn update_negative_flag(&mut self, result: u8) {
        let ret = result & 0b1000_0000;
        if ret != 0 {
            self.status.Z = true;
        } else {
            self.status.Z = false;
        }
    }

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.reg_a, 5);
        // assert!(cpu.status & 0b0000_0010 == 0);
        // assert!(cpu.status & 0b1000_0000 == 0);
        assert!(cpu.status.Z == false);
        assert!(cpu.status.N == false);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        //assert!(cpu.status & 0b0000_0010 == 0b10);
        assert!(cpu.status.Z == true);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A,0xaa, 0x00]);

        assert_eq!(cpu.reg_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa,0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.reg_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.reg_a, 0x55);
    }
}