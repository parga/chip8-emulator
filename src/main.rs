use chip8::Chip8;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs::File, io::Read}; // Adjust this import as needed

const TIMER_HZ: u64 = 60;
const INSTRUCTION_HZ: u64 = 700; // Typical value, adjust as needed
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

    let mut chip8 = Chip8::new();
    chip8.load_rom(data);

    let timer_interval = Duration::from_secs_f64(1.0 / TIMER_HZ as f64);
    let instruction_interval = Duration::from_secs_f64(1.0 / INSTRUCTION_HZ as f64);

    let mut last_timer = Instant::now();
    let mut last_instruction = Instant::now();
    loop {
        if last_instruction.elapsed() >= instruction_interval {
            chip8.run_instruction();
            last_instruction = Instant::now();
        }

        // Tick timers at TIMER_HZ
        if last_timer.elapsed() >= timer_interval {
            chip8.tick(); // This should decrement timers in your Bus/Clock
            last_timer = Instant::now();
        }

        // Optionally sleep a tiny bit to avoid busy-waiting
        sleep(Duration::from_micros(100));
    }
}
