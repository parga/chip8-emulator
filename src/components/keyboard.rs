// pub trait Chip8Keyboard {
//     fn is_key_pressed(&self, key: u8) -> bool;
//     // You can add more methods as needed, e.g., wait_for_keypress()
// }

#[derive(Default)]
pub struct Keyboard {
    keys: u16,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: 0 }
    }

    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        if key_code < 16 {
            (self.keys & (1 << key_code)) != 0
        } else {
            false
        }
    }

    pub fn set_keys(&mut self, pressed_keys: u16) {
        self.keys = pressed_keys;
    }

    pub fn get_key_blocking(&self) -> Option<u8> {
        (0..16).find(|&key_code| self.is_key_pressed(key_code))
        // panic!("No key is currently pressed.");
    }
}

