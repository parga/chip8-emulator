use std::{fs::File, io::Read};

use chip8::Chip8;
use chip8::components::keyboard::Keyboard;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    let number_of_bits = file.read_to_end(&mut data);
    match number_of_bits {
        Ok(n) => {
            println!("Read {n} bytes.");
        }
        Err(e) => println!("Error reading file: {e}"),
    }

    let keyboard = Keyboard::new();
    let mut chip8 = Chip8::new();
    chip8.load_rom(data);

    loop {
        chip8.run_instruction(&keyboard);
    }
}
