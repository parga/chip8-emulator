#[derive(Debug)]
pub struct Ram {
    mem: [u8; 4096],
    pub sp: u8, // stack pointer
}
const STACK_INITIAL_INDEX: u16 = 0xEA0;
impl Ram {
    pub fn new() -> Self {
        let mut ram = Ram {
            mem: [0; 4096],
            sp: 0,
        };

        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        let mut i = 0;
        for sprite in sprites.iter() {
            for ch in sprite {
                ram.mem[i] = *ch;
                i += 1;
            }
        }

        // for i in 0..0x1ff {
        //     print!("{:#X} ", ram.mem[i]);
        // }
        ram
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn push_to_stack(&mut self, address_value: u16) {
        println!("writing the address {:X} into the stack", address_value);
        let current_stack_address = STACK_INITIAL_INDEX + (self.sp as u16);
        let hi = ((0xFF00 & address_value)>> 8) as u8;
        let lo = (0xFF & address_value) as u8;
        self.write_byte(current_stack_address, hi);
        self.write_byte(current_stack_address + 1, lo);
        self.sp += 2;
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        let current_stack_address = STACK_INITIAL_INDEX + (self.sp as u16);
        let hi = self.read_byte(current_stack_address - 2) as u16;
        let lo = self.read_byte(current_stack_address - 1) as u16;
        self.sp -= 2;
        println!(
            "comming back to address hi:{:X} lo:{:X} =  {:X}",
            hi << 8,
            lo,
            (hi << 8) | lo
        );
        (hi << 8) | lo
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}
