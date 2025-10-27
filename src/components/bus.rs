use super::display::Display;
use super::keyboard::Keyboard;
use super::ram::Ram;
use std::fmt::{self, Debug};

#[derive(Default)]
pub struct Clock {
    delay: u8,
    sound: u8,
}

impl Clock {
    pub fn new() -> Self {
        Self { delay: 0, sound: 0 }
    }
    pub fn set_delay_timer(&mut self, delay: u8) {
        self.delay = delay;
    }
    pub fn get_delay_timer(&self) -> u8 {
        self.delay
    }
    pub fn set_sound_timer(&mut self, sound: u8) {
        self.sound = sound;
    }
    pub fn get_sound_timer(&self) -> u8 {
        self.sound
    }
    pub fn tick(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }
}

#[derive(Default)]
pub struct Bus {
    pub ram: Ram,
    keyboard: Keyboard,
    display: Display,
    clock: Clock,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: Ram::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            clock: Clock::new(),
        }
    }
    pub fn ram_read_byte(&mut self, address: u16) -> u8 {
        self.ram.read_byte(address)
    }

    pub fn ram_write_byte(&mut self, address: u16, value: u8) {
        self.ram.write_byte(address, value);
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        self.display.debug_draw_byte(byte, x, y)
    }

    pub fn present_screen(&self) {
        self.display.present_screen_to_terminal();
    }

    pub fn clear_screen(&mut self) {
        self.display.clear_screen();
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

    pub fn set_delay_timer(&mut self, delay: u8) {
        self.clock.set_delay_timer(delay)
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.clock.get_delay_timer()
    }

    pub fn set_sound_timer(&mut self, sound: u8) {
        self.clock.set_sound_timer(sound)
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.clock.get_sound_timer()
    }

    pub fn tick(&mut self) {
        self.clock.tick()
    }

    pub fn get_display_buffer(&self) -> Vec<u32> {
        self.display.get_display_buffer() 
    }

    pub fn set_keys(&mut self, pressed_keys: u16) {
        self.keyboard.set_keys(pressed_keys);
    }
    pub fn get_key_blocking(&self) -> Option<u8> {
        self.keyboard.get_key_blocking()
    }
}

impl Debug for Bus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Delay timer: {:?} ", self.clock.get_delay_timer()).unwrap();
        write!(f, "Keyboard: {:X}  ", self.keyboard.keys).unwrap();
        write!(f, "Keyboard: {:X}", self.clock.get_sound_timer()).unwrap();
        Ok(())
    }
}
