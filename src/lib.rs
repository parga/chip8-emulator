pub mod components;

use components::cpu::Cpu;
use components::bus::Bus;


pub struct Chip8 {
    bus: Bus,
    cpu: Cpu
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 { bus: Bus::new(), cpu: Cpu::new() }
    }

    pub fn load_rom(&mut self, data : Vec<u8>) {
        let offset = 0x200;
        (0..data.len()).for_each(|i| {
            self.bus.ram_write_byte(offset + i as u16, data[i]);
        });
    }

    pub fn run_instruction(&mut self) {
        self.bus.tick();
        self.cpu.run_instruction(&mut self.bus);
        println!("Cpu state -------------------- ");
        println!("{:?}", self.cpu);
        println!("{:?}", self.bus) 
    }

    pub fn tick(&mut self) {
        self.bus.tick();
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
