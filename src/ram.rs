pub struct Ram {
    mem: [u8; 4096],
}

impl Ram {
    pub fn new() -> Self {
        Ram { mem: [0; 4096] }
    }

    pub fn write_byte(&mut self, _address: u16, _value: u8) {
        unimplemented!()
    }

    pub fn read_byte(&mut self, _address: u16, _value: u8) {
        unimplemented!()
    }
}

#[warn(dead_code)]
impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}
