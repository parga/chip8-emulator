use std::fmt::{self, Debug};

use crate::components::ram::Ram;

const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
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
        let nn = instruction & 0x00FF; // lowest 8 bits
        let n = instruction & 0x000F; // lowest 4 bits
        let x = (instruction & 0x0F00) >> 8; // bits 8-11
        let y = (instruction & 0x00F0) >> 4; // bits 4-7

        println!("nn={:?}, nn={:?}, n={:?}, x={}, y={}", nnn, nn, n, x, y);
        match (instruction & 0xF000) >> 12 {
            0x0 => match instruction & 0x00FF {
                0xE0 => {}
                0xEE => {}
                _ => panic!("unrecognized 0x00xx instruction"),
            },
            0x1 => {
                // jump NNN
                self.pc = nnn
            },
            0x6 => {
                // vx := NN
                self.vx[x as usize] = nn as u8;
            }
            _ => panic!("unrecognized intruction {:#X}:{:#X}", self.pc, instruction),
        }
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
            .field("vx", &self.vx)
            .field("pc", &self.pc)
            .field("i", &self.i)
            .finish()
    }
}
