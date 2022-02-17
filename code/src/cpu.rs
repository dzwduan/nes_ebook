pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub pc: u16,
}


impl CPU {
    pub fn new() -> Self {
        CPU { register_a: 0, register_x:0, status: 0, pc: 0 }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        //todo!(r#"not implemented yet"#)

        self.pc = 0;

        loop {
            let opcode = program[self.pc as usize];
            self.pc += 1;

            match opcode {
                //LDA
                0xA9 => {
                    let param = program[self.pc as usize];
                    self.pc += 1;
                    self.register_a = param;
                    
                    //set zero flag
                    //N	V	–	B	D	I	Z	C
                    if self.register_a == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }

                    //set negative flag
                    if self.register_a & 0b1000_0000 !=0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }
                }

                0xAA => {
                    self.register_x = self.register_a;

                    if self.register_x == 0 {
                        self.status = self.status | 0b0000_0010;
                    } else {
                        self.status = self.status & 0b1111_1101;
                    }
    
                    if self.register_x & 0b1000_0000 != 0 {
                        self.status = self.status | 0b1000_0000;
                    } else {
                        self.status = self.status & 0b0111_1111;
                    }
                }

                //BRK
                0x00 => {
                    return;
                }
                _ => todo!()
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);

        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }


    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);
  
        assert_eq!(cpu.register_x, 10)
    }
}