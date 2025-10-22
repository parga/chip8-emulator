const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [u8; WIDTH * HEIGHT],
}

impl Default for Display {
    fn default() -> Self {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }
}

impl Display {
    pub fn new() -> Self {
        Display {
            screen: [0; WIDTH * HEIGHT],
        }
    }

    pub fn get_index_from_coord(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }
    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut flipped = false;
        let mut cord_x = x as usize;
        let cord_y = y as usize;
        let mut b = byte;

        for _ in 0..8 {
            if cord_x >= WIDTH || cord_y >= HEIGHT {
                break;
            }
            let index = Display::get_index_from_coord(cord_x, cord_y);
            match (b & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[index] == 1 {
                        flipped = true
                    }
                    self.screen[index] = 0;
                }
                1 => self.screen[index] = 1,
                _ => unreachable!(),
            }
            cord_x += 1;
            b <<= 1;
        }
        flipped
    }
    pub fn present_screen(&self) {
        for index in 0..self.screen.len() {
            let pixel = self.screen[index];

            if index % WIDTH == 0 {
                println!();
            }
            match pixel {
                0 => print!("_"),
                1 => print!("#"),
                _ => unreachable!(),
            }
        }
        println!();
        // for y in 0..HEIGHT {
        //     for x in 0..WIDTH {
        //         let index = Display::get_index_from_coord(x, y);
        //         if self.screen[index] == 0 {
        //             print!(" ");
        //         } else {
        //             print!("@");
        //         }
        //     }
        //     println!();
        // }
    }

    pub fn clear_screen(&mut self) {
        self.screen = [0; WIDTH * HEIGHT];
    }

    pub fn get_display_buffer(&self) -> Vec<u32> {
        self.screen.iter().map(|&b| b as u32).collect()
        // &self.screen
    }
}
