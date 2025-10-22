use chip8::Chip8;
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use std::{fs::File, io::Read}; // Adjust this import as needed

const TIMER_HZ: u64 = 60;
const INSTRUCTION_HZ: u64 = 500; // Typical value, adjust as needed
fn scale_buffer(buffer: &[u32], width: usize, height: usize, scale: usize) -> Vec<u32> {
    let scaled_width = width * scale;
    let scaled_height = height * scale;
    let mut scaled = vec![0u32; scaled_width * scaled_height];

    for y in 0..height {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            for dy in 0..scale {
                for dx in 0..scale {
                    let sx = x * scale + dx;
                    let sy = y * scale + dy;
                    scaled[sy * scaled_width + sx] = pixel;
                }
            }
        }
    }
    scaled
}

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

    let width = 64;
    let height = 32;
    let scale = 5;
    let mut window = Window::new(
        "Chip 8",
        width * scale,
        height * scale,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let timer_interval = Duration::from_secs_f64(1.0 / TIMER_HZ as f64);
    let instruction_interval = Duration::from_secs_f64(1.0 / INSTRUCTION_HZ as f64);

    let mut last_timer = Instant::now();
    let mut last_instruction = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut needs_buffer_refresh = false;
        if last_instruction.elapsed() >= instruction_interval {
            needs_buffer_refresh = chip8.run_instruction();
            last_instruction = Instant::now();
        }

        if last_timer.elapsed() >= timer_interval {
            chip8.tick(); // Decrement timers
            last_timer = Instant::now();
        }

        if needs_buffer_refresh {
            let chip8_buffer = chip8.get_display_buffer();
            let color_buffer: Vec<u32> = chip8_buffer
                .iter()
                .map(|&b| if b == 0 { 0xFF000000 } else { 0xFFFFFFFF })
                .collect();
            let scaled_buffer = scale_buffer(&color_buffer, width, height, scale);
            window
                .update_with_buffer(&scaled_buffer, width * scale, height * scale)
                .unwrap();
        }

        // Sleep a bit to avoid busy-waiting
        std::thread::sleep(Duration::from_micros(100));
    }
}
