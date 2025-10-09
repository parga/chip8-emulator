use std::{fs::File, io::Read};

use chip8::Chip8;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    let number_of_bits = file.read_to_end(&mut data);
    match number_of_bits {
        Ok(n) => {
            print!("Read {n} bytes.");
            return ()
        }
        Err(e) => println!("Error reading file: {e}"),
    }

    let mut chip8 = Chip8::new();
    chip8.load_rom(data);
}
