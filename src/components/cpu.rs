use std::fmt::{self, Debug};

use crate::components::ram::Ram;

const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
        }
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {
        let hi = ram.read_byte(self.pc) as u16;
        let lo = ram.read_byte(self.pc + 1) as u16;
        let instruction = (hi << 8) | lo;
        println!(
            "Instruction Read {:#X}, lo: {:#X}, hi: {:#X}",
            instruction, lo, hi
        );

        let nnn = instruction & 0x0FFF; // lowest 12 bits
        let nn = (instruction & 0x00FF) as u8; // lowest 8 bits
        let n = (instruction & 0x000F) as u8; // lowest 4 bits
        let x = ((instruction & 0x0F00) >> 8) as u8; // bits 8-11
        let y = ((instruction & 0x00F0) >> 4) as u8; // bits 4-7

        if self.prev_pc == self.pc {
            panic!("Stuck in infinite loop at pc={:#X}", self.pc);
        }
        self.prev_pc = self.pc;

        println!("nn={:?}, nn={:?}, n={:?}, x={}, y={}", nnn, nn, n, x, y);
        match (instruction & 0xF000) >> 12 {
            0x0 => match instruction & 0x00FF {
                0xE0 => {}
                0xEE => {}
                _ => panic!("unrecognized 0x00xx instruction"),
            },
            0x1 => {
                // jump NNN
                self.pc = nnn;
                self.pc -= 2; // compensate for pc += 2 at the end
            }
            0x6 => {
                // vx := NN
                self.write_reg_vx(x, nn)
            }
            0xD => {
                self.debug_drawn_sprite(ram, x, y, n); // draw sprite at (Vx, Vy) with height N
            }
            0xA => {
                // I := NNN
                println!("Set I to {:#X}", nnn);
                self.i = nnn;
            }
            _ => panic!(
                "unrecognized intruction :{:#X} in pc :{:#X}",
                instruction, self.pc
            ),
        }
        self.pc += 2;
    }

    fn write_reg_vx(&mut self, x: u8, value: u8) {
        println!("Set V{:#X} to {:#X}", x, value);
        self.vx[x as usize] = value;
    }

    fn read_reg_vx(&mut self, index: u16) -> u8 {
        let value = self.vx[index as usize];
        println!("Reading value at {:#X} to {:#X}", index, value);
        value
    }

    fn debug_drawn_sprite(&self, ram: &mut Ram, x: u8, y: u8, height: u8) {
        for row_index in 0..height {
            let mut b = ram.read_byte((self.i as u8 + row_index + y) as u16);
            // for _ in 0..8 {
            //     match (b & 0b1000_0000) >> 7 {
            //         0 => print!("_"),
            //         1 => print!("#"),
            //         _ => unreachable!(),
            //     }
            //     b <<= 1;
            // }
            println!("Sprite row {:#X}: {:#X}", (self.i as u8 + row_index +y) , b);
            println!();
        }
        println!();
        // println!("Drawing sprite of height {:#X}", height);
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "pc: {:X}", self.pc).unwrap();
        write!(f, "vx: ").unwrap();
        for item in self.vx.iter() {
            write!(f, "{:#X} ", item).unwrap();
        }
        writeln!(f).unwrap();
        write!(f, "i: {:#X}", self.i).unwrap();
        writeln!(f).unwrap();

        Ok(())
    }
}
