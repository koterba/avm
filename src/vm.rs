use crate::instructions::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    pub program: Vec<u8>,
    pub equal_flag: bool,
    pub remainder: u32,
    pub heap: Vec<u8>,
    pub pc: usize // program counter
}


impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            equal_flag: false,
            program: vec![],
            remainder: 0,
            heap: vec![],
            pc: 0
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }
    
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            },
            Opcode::HLT => {
                println!("HLT encountered.");
                return true;
            },
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            },
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            },
            Opcode::JMPF => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc += value as usize;
            },
            Opcode::JMPB => {
                let value = self.registers[self.next_8_bits() as usize];
                self.pc -= value as usize;
            },
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false
                }
                self.next_8_bits();
            },
            Opcode::JEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            },
            Opcode::JNEQ => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if !self.equal_flag {
                    self.pc = target as usize;
                }
            },
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            },
            Opcode::INC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] += 1;
            },
            Opcode::DEC => {
                let register = self.next_8_bits() as usize;
                self.registers[register] -= 1;
            },
            Opcode::IGL => {
                println!("Invalid opcode encountered... Terminating.");
                return true;
            }
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        opcode
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        result
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        result
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        test_vm.program = vec![1,0,0,0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        test_vm.program = vec![200,0,0,0];
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0, 0, 0, 10,
            0, 1, 0, 3,
            4, 0, 1, 2
        ];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 30);
    }

    #[test]
    fn test_opcode_jmp() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4)
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = VM::new();
        test_vm.program = vec![
            0, 0, 0, 50, // load 50 into register 0
            0, 1, 0, 51, // load 51 into register 1
            9, 0, 1, 0   // compare register 1 and 0
        ];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false)
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![10, 0, 0, 0, 0, 0, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 7)
    }

    #[test]
    fn test_opcode_aloc() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 7;
        test_vm.program = vec![12, 0]; // extend heap by val in reg 0
        test_vm.run_once();
        assert_eq!(test_vm.heap.len(), 7)
    }
}