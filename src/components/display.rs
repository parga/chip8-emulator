pub struct Display {
    screen: [[u8; 64]; 32],
    width: u8,
    height: u8
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [[0; 64]; 32],
            width: 64,
            height: 32,
        }
    }

    pub fn debug_draw_byte(&self, mut byte: u8, _x: u8, _y: u8) {
        for _ in 0..8 {
            match (byte & 0b1000_0000) >> 7 {
                0 => print!("_"),
                1 => print!("#"),
                _ => unreachable!(),
            }
            byte <<= 1;
        }
        println!();
    }
}
