use super::display::Display;
use super::ram::Ram;
use super::keyboard::Keyboard;

#[derive(Default)]
pub struct Bus {
    pub ram: Ram,
    keyboard: Keyboard,
    display: Display
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(), 
            display: Display::new(),
            keyboard: Keyboard::new()
        }
    }
    pub fn ram_read_byte(&mut self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn ram_write_byte(&mut self, address: u16, value: u8 ) {
        self.ram.write_byte(address, value);
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) {
        self.display.debug_draw_byte(byte, x, y);
    }

    pub fn push_to_stack(&mut self, address_value: u16) {
        self.ram.push_to_stack(address_value);
    }
    pub fn pop_from_stack(&mut self) -> u16 {
        self.ram.pop_from_stack()
    }

    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        self.keyboard.is_key_pressed(key_code)
    }
}
