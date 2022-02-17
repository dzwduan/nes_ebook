use crate::cpu::CPU;

pub mod cpu;

fn main() {
    let mut cpu = CPU::new();
    cpu.interpret(vec![0x00,0x11,0x22]);
}
