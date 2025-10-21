#[derive(Default)]
pub struct Display {
    screen: [[]]
}

impl Display {
    pub fn new() -> Self {
        Display {}
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
