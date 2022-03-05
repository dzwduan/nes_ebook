// use crate::cpu::CPU;

pub mod cpu;
pub mod opcodes;

#[macro_use]
extern crate lazy_static;
extern crate core;

fn main() {
    // let mut cpu = CPU::new();
    // cpu.interpret(vec![0x00,0x11,0x22]);
}
