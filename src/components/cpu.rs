use crate::components::ram::Ram;


const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
}


impl Cpu {
    pub fn new() -> Self {
        Self {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {
        ram.read_byte(self.pc);
    }
}
