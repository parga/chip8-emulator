// pub trait Chip8Keyboard {
//     fn is_key_pressed(&self, key: u8) -> bool;
//     // You can add more methods as needed, e.g., wait_for_keypress()
// }

#[derive(Default)]
pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        if key_code < 16 {
            self.keys[key_code as usize]
        } else {
            false
        }
    }
}

