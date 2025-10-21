
pub trait Chip8Keyboard {
    fn is_key_pressed(&self, key: u8) -> bool;
    // You can add more methods as needed, e.g., wait_for_keypress()
}

#[derive(Debug, Default)]
pub struct Keyboard {
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }
}

impl Chip8Keyboard for Keyboard {
    fn is_key_pressed(&self, key: u8) -> bool {
        if key < 16 {
            self.keys[key as usize]
        } else {
            false
        }
    }
}
