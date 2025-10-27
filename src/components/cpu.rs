use std::fmt::{self, Debug};

use crate::components::bus::Bus;
const PROGRAM_START: u16 = 0x200;

#[derive(Default)]
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

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let hi = bus.ram_read_byte(self.pc) as u16;
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
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

        if self.prev_pc == self.pc && ((instruction & 0xF0FF) != 0xF00A) {
            panic!("Stuck in infinite loop at pc={:#X}", self.pc);
        }

        self.prev_pc = self.pc;

        // println!(
        //     "nnn={:X}, nn={:X}, n={:X}, x={:X}, y={:X}, sp={}",
        //     nnn, nn, n, x, y, bus.ram.sp
        // );
        match (instruction & 0xF000) >> 12 {
            0x0 => match nn {
                0xE0 => {
                    // clear the display
                    bus.clear_screen();
                }
                0xEE => {
                    // return from function
                    let address = bus.pop_from_stack();
                    self.pc = address;
                    self.pc -= 2; // compensate for pc += 2 at the end
                }
                _ => panic!("unrecognized 0x00xx instruction"),
            },
            0x1 => {
                // jump NNN
                self.pc = nnn;
                self.pc -= 2; // compensate for pc += 2 at the end
            }
            0x2 => {
                // start subroutine at nnn
                bus.push_to_stack(self.pc + 2);
                self.pc = nnn;
                self.pc -= 2; // compensate for pc += 2 at the end
            }
            0x3 => {
                // if vx == nn skip next instruction
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 2;
                }
            }
            0x4 => {
                // if vx != nn skip next instruction
                let vx = self.read_reg_vx(x);
                if vx != nn {
                    self.pc += 2;
                }
            }
            0x6 => {
                // vx := NN
                self.write_reg_vx(x, nn)
            }
            0x7 => {
                // vx += nn
                let mut vx = self.read_reg_vx(x);
                vx = vx.wrapping_add(nn);
                self.write_reg_vx(x, vx);
            }
            0x8 => {
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);

                match n {
                    0x0 => {
                        // vx = vy
                        self.write_reg_vx(x, vy);
                    }
                    0x1 => {
                        // vx = vx | vy
                        self.write_reg_vx(x, vx | vy);
                    }
                    0x2 => {
                        // vx = vx & vy
                        self.write_reg_vx(x, vx & vy);
                    }
                    0x3 => {
                        // vx = vx^vy
                        self.write_reg_vx(x, vx ^ vy);
                    }
                    0x4 => {
                        // vx += vy
                        let (result, carry) = vx.overflowing_add(vy);
                        self.write_reg_vx(x, result);
                        self.write_reg_vx(0xF, if carry { 1 } else { 0 });
                    }
                    0x5 => {
                        // vx -= vy
                        self.write_reg_vx(0xF, if vx > vy { 1 } else { 0 });
                        self.write_reg_vx(x, vx.wrapping_sub(vy));
                    }
                    0x6 => {
                        // vx = vx >> 1
                        self.write_reg_vx(0xF, vx & 0x1);
                        self.write_reg_vx(x, vx >> 1);
                    }
                    0x7 => {
                        // vx = vy - vx
                        self.write_reg_vx(0xF, if vy > vx { 1 } else { 0 });
                        self.write_reg_vx(x, vy.wrapping_sub(vx));
                    }
                    0x8 => {
                        // vx = vx << 1
                        self.write_reg_vx(0xF, (vx & 0b1000_0000) >> 7);
                        self.write_reg_vx(x, vx << 1);
                    }

                    _ => unreachable!(),
                }
            }
            0x9 => {
                // if vx != vy skip next instruction
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                if vx != vy {
                    self.pc += 2;
                }
            }
            0xA => {
                // I := NNN
                println!("Set I to {:#X}", nnn);
                self.i = nnn;
            }
            0xC => {
                // vx = random byte AND nn
                let random_byte: u8 = rand::random();
                let result = random_byte & nn;
                self.write_reg_vx(x, result);
            }
            0xD => {
                // draw sprite at (Vx, Vy) with height N
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);
                self.debug_drawn_sprite(bus,vx, vy, n);
            }
            0xE => match nn {
                0xA1 => {
                    // skip next instruction if key in the value of vx is not pressed
                    if !bus.is_key_pressed(self.read_reg_vx(x)) {
                        self.pc += 2
                    }
                }
                0x9E => {
                    if bus.is_key_pressed(self.read_reg_vx(x)) {
                        self.pc += 2
                    }
                }
                _ => unreachable!(),
            },
            0xF => match nn {
                0x1E => {
                    // add vx to i wrapping
                    let vx = self.read_reg_vx(x);
                    self.i = self.i.wrapping_add(vx as u16);
                }
                0x65 => {
                    // `LD V0~Vx, [I]`
                    // store what is in memory startin in I into vx
                    for offset in 0..=x {
                        let value = bus.ram_read_byte(self.i + offset as u16);
                        self.write_reg_vx(offset, value);
                    }
                }
                0x15 => {
                    // Set delay timer to Vx
                    let vx = self.read_reg_vx(x);
                    bus.set_delay_timer(vx);
                }
                0x07 => {
                    self.write_reg_vx(x, bus.get_delay_timer());
                }
                0x0A => {
                    // wait for a key press and store it in vx
                    let key = bus.get_key_blocking();
                    match key {
                        Some(k) => {
                            self.write_reg_vx(x, k);
                        }
                        None => {
                            // no key pressed, repeat this instruction
                            self.pc -= 2;
                        }
                    }
                }
                0x18 => {
                    // set sound timer to vx
                    let vx = self.read_reg_vx(x);
                    bus.set_sound_timer(vx);
                }
                0x33 => {
                    // store bcd representation of vx in memory at I, I+1, I+2
                    let vx = self.read_reg_vx(x);
                    let hundreds = vx / 100;
                    let tens = (vx % 100) / 10;
                    let units = vx % 10;
                    bus.ram_write_byte(self.i, hundreds);
                    bus.ram_write_byte(self.i + 1, tens);
                    bus.ram_write_byte(self.i + 2, units);
                }
                0x29 => {
                    // set I to the location of the sprite for the character in vx
                    let vx = self.read_reg_vx(x);
                    let sprite_address = (vx as u16) * 5; // each sprite is 5 bytes long
                    self.i = sprite_address;
                }
                _ => unreachable!(),
            },
            _ => panic!(
                "unrecognized intruction :{:#X} in pc :{:#X}",
                instruction, self.pc
            ),
        }
        self.pc += 2;
    }

    fn write_reg_vx(&mut self, x: u8, value: u8) {
        println!("Set V{:X} = {:#X}", x, value);
        self.vx[x as usize] = value;
    }

    fn read_reg_vx(&mut self, index: u8) -> u8 {
        let value = self.vx[index as usize];
        println!(
            "Reading value in registry V{:X} with value {:#X}",
            index as usize, value
        );
        value
    }

    fn debug_drawn_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        println!("Drawing byte at position x:{}, y:{}", x, y);
        let mut should_set_vf = false;
        for row in 0..height {
            let b = bus.ram_read_byte(self.i + row as u16);
            if bus.debug_draw_byte(b, x, y + row) {
                should_set_vf = true;
            }
        }

        // if any if the pixels was changed from set to unset
        // we set VF to 1
        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        } else {
            self.write_reg_vx(0xF, 0);
        }
        // bus.present_screen();
        println!();
    }
}

impl Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "pc: {:X}", self.pc).unwrap();
        write!(f, "vx: ").unwrap();
        for (i, item) in self.vx.iter().enumerate() {
            write!(f, "V{:X}={:#X} ", i as u8, item).unwrap();
        }
        writeln!(f).unwrap();
        write!(f, "i: {:#X}", self.i).unwrap();
        writeln!(f).unwrap();

        Ok(())
    }
}
