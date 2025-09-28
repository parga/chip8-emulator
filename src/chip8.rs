use crate::ram::Ram;

pub struct Chip8 {
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 { ram: Ram::new() }
    }

    pub fn load_rom(&mut self, data : Vec<u8>) {
        let offset = 0x200;
        (0..data.len()).for_each(|i| {
            self.ram.write_byte(offset + i as u16, data[i]);
        });
    }
}

#[warn(dead_code)]
impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}
