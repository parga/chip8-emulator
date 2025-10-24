pub mod components;

use components::bus::Bus;
use components::cpu::Cpu;
use minifb::Key;

pub struct Chip8 {
    bus: Bus,
    cpu: Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        let offset = 0x200;
        (0..data.len()).for_each(|i| {
            self.bus.ram_write_byte(offset + i as u16, data[i]);
        });
    }

    pub fn run_instruction(&mut self) -> bool {
        self.bus.tick();
        let needs_buffer_refresh = self.cpu.run_instruction(&mut self.bus);
        println!("Cpu state -------------------- ");
        println!("{:?}", self.cpu);
        println!("{:?}", self.bus);
        needs_buffer_refresh
    }

    pub fn tick(&mut self) {
        self.bus.tick();
    }

    pub fn get_display_buffer(&self) -> Vec<u32> {
        self.bus.get_display_buffer()
    }

    pub fn set_keys(&mut self, pressed_keys: u16) {
        self.bus.set_keys(pressed_keys);
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
