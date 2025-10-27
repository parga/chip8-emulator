use chip8::Chip8;
use minifb::{Key, Window, WindowOptions};
use rodio::source::SineWave;
use rodio::{OutputStream, OutputStreamBuilder, Sink};
use std::time::{Duration, Instant};
use std::{fs::File, io::Read};

const TIMER_HZ: u64 = 60;
const INSTRUCTION_HZ: u64 = 500;

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

fn map_window_pressed_keys_to_chip8_u16(window_key_pressed: Vec<Key>) -> u16 {
    let chip8_key_map = [
        Key::X,    // 0
        Key::Key1, // 1
        Key::Key2, // 2
        Key::Key3, // 3
        Key::Q,    // 4
        Key::W,    // 5
        Key::E,    // 6
        Key::A,    // 7
        Key::S,    // 8
        Key::D,    // 9
        Key::Z,    // A
        Key::C,    // B
        Key::Key4, // C
        Key::R,    // D
        Key::F,    // E
        Key::V,    // F
    ];

    let mut bitmask = 0u16;
    for (chip8_idx, &mapped_key) in chip8_key_map.iter().enumerate() {
        if window_key_pressed.contains(&mapped_key) {
            bitmask |= 1 << chip8_idx
        }
    }
    bitmask
}

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    // let mut file = File::open("data/pong.rom").unwrap();
    // let mut file = File::open("data/tetris.rom").unwrap();
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
    let scale = 10;
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

    let stream_handle =
        OutputStreamBuilder::open_default_stream().expect("Failed to open output stream");
    let mut sink: Sink = Sink::connect_new(stream_handle.mixer());
    let mut beep_playing = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.set_keys(map_window_pressed_keys_to_chip8_u16(window.get_keys()));
        if last_instruction.elapsed() >= instruction_interval {
            chip8.run_instruction();
            last_instruction = Instant::now();
        }

        if last_timer.elapsed() >= timer_interval {
            chip8.tick(); // Decrement timers

            // Sound logic
            let sound_timer = chip8.get_sound_timer();
            if sound_timer > 0 {
                if !beep_playing {
                    sink = Sink::connect_new(stream_handle.mixer());
                    sink.append(SineWave::new(440_f32));
                    beep_playing = true;
                }
            } else if beep_playing {
                sink.stop();
                beep_playing = false;
            }

            // Display update
            let chip8_buffer = chip8.get_display_buffer();
            let color_buffer: Vec<u32> = chip8_buffer
                .iter()
                .map(|&b| if b == 0 { 0xFF000000 } else { 0xFFFFFFFF })
                .collect();
            let scaled_buffer = scale_buffer(&color_buffer, width, height, scale);
            window
                .update_with_buffer(&scaled_buffer, width * scale, height * scale)
                .unwrap();

            last_timer = Instant::now();
        }

        // Sleep a bit to avoid busy-waiting
        std::thread::sleep(Duration::from_micros(100));
    }
}
