// pub trait Chip8Keyboard {
//     fn is_key_pressed(&self, key: u8) -> bool;
//     // You can add more methods as needed, e.g., wait_for_keypress()
// }

#[derive(Default)]
pub struct Keyboard {
    pub keys: u16,
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
        if self.keys != 0 {
            for key_code in 0..16 {
                if (self.keys & (1 << key_code)) != 0 {
                    return Some(key_code);
                }
            }
        }
        None
    }
}

