const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [[u8; WIDTH]; HEIGHT],
}

impl Default for Display {
    fn default() -> Self {
        Display {
            screen: [[0; WIDTH]; HEIGHT],
        }
    }
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut flipped = false;
        let mut cord_x = x as usize;
        let cord_y = y as usize;
        let mut b = byte;

        for _ in 0..8 {
            match (b & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[cord_y][cord_x]== 1 {
                        flipped = true
                    }
                    self.screen[cord_y][cord_x] = 0;
                }
                1 => self.screen[cord_y][cord_x] = 1,
                _ => unreachable!(),
            }
            cord_x += 1;
            b <<= 1;
        }
        flipped
    }
    pub fn present_screen(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.screen[y][x] == 0 {
                    print!("_");
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen = [[0; WIDTH]; HEIGHT];
    } 
}
