pub mod components;

use components::ram::Ram;
use components::cpu::Cpu;
use components::keyboard::Chip8Keyboard;


pub struct Chip8 {
    ram: Ram,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 { ram: Ram::new(), cpu: Cpu::new() }
    }

    pub fn load_rom(&mut self, data : Vec<u8>) {
        let offset = 0x200;
        (0..data.len()).for_each(|i| {
            self.ram.write_byte(offset + i as u16, data[i]);
        });
    }

    pub fn run_instruction(&mut self, keyboard: &dyn Chip8Keyboard) {
        self.cpu.run_instruction(&mut self.ram, keyboard);
        println!("Cpu state -------------------- \n{:?}", self.cpu) 
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
