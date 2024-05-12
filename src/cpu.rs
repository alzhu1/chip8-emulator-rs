use std::{fs::File, io::{BufReader, Read}};

use rand;

// TODO: To support CHIP-8 variants, could introduce an enum to Chip8 struct

// TODO: I think this whole thing is the CPU itself? So loop should be done outside of this?
// Or the loop should be done in a Chip8 APP? Or this is renamed as CPU?

const FONT_LOCATION: u16 = 0x0;
const FONT_BYTES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

// Entry point to chip8 emulator
pub struct CPU {
    pub pixels: [[bool; 64]; 32],
    pub memory: [u8; 4096],

    V: [u8; 0x10], // registers
    I: u16,       // 12-bit index reg
    pc: usize,
    delay_timer: u8,
    sound_timer: u8,

    // TODO: Is this the best place to put this?
    stack: [usize; 16],
    sp: usize,

    // TODO: Array or bitmask?
    key: u16
}

impl Default for CPU {
    fn default() -> Self {
        let mut memory = [0; 4096];
        for (i, &font_byte) in FONT_BYTES.iter().enumerate() {
            memory[FONT_LOCATION as usize + i] = font_byte;
        }

        let file = File::open("test_rom.ch8").unwrap();
        for (i, a) in BufReader::new(file).bytes().enumerate() {
            memory[0x200 + i] = a.unwrap();
        }

        // for (i, rm) in test_rom.iter().enumerate() {
        //     let offset = i * 2;

        //     let temp = rm.to_be_bytes();
        //     memory[0x200 + offset] = temp[0];
        //     memory[0x200 + offset + 1] = temp[1];
        // }

        let pixels = [[false; 64]; 32];

        Self {
            pixels,
            memory,
            V: [0; 0x10],
            I: 0,
            // TODO: Some programs can start elsewhere
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: 0
        }
    }
}

// impl CPU {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     // Consume after use?
//     pub fn run(mut self) {
//         loop {
//             self.test();
//         }
//     }

//     fn test(&mut self) {
//         self.memory[0] = 1;
//     }
// }

impl CPU {
    pub fn process(&mut self) {
        let upper = self.memory[self.pc];
        let lower = self.memory[self.pc + 1];
        let instruction = ((upper as u16) << 8) | (lower as u16);

        let (b1, b2) = (upper >> 4, upper & 0xF);
        let (b3, b4) = (lower >> 4, lower & 0xF);

        // let [_upper, lower] = instruction.to_be_bytes();
        // let [b1, b2, b3, b4] = [12, 8, 4, 0].map(|b| ((instruction.to_be() >> b) & 0xF) as u8);


        // println!("PC = {:#x}, Instruction: {:#x}", self.pc, instruction);

        // Increment here, so that jumps aren't affected
        self.pc += 2;


        match b1 {
            0 => {
                match lower {
                    0xE0 => self.clear_screen(),
                    0xEE => self.return_subr(),
                    _ => self.sys(instruction & 0xFFF)
                }
            },
            1 => self.jmp(instruction & 0xFFF),
            2 => self.call(instruction & 0xFFF),
            3 => self.cond_check(b2, (lower, false), true),
            4 => self.cond_check(b2, (lower, false), false),
            5 if b4 == 0 => self.cond_check(b2, (b3, true), true),
            6 => self.set_immediate(b2, lower),
            7 => self.add_immediate(b2, lower),
            8 => self.execute_arithmetic(b2, b3, b4),
            9 => self.cond_check(b2, (b3, true), false),
            0xA => self.set_i(instruction & 0xFFF),
            0xB => self.jmp_relative(instruction & 0xFFF),
            0xC => self.set_random(b2, lower),
            0xD => self.draw(b2, b3, b4),
            0xE => {
                match lower {
                    0x9E => self.key_check(b2, true),
                    0xA1 => self.key_check(b2, false),
                    _ => unreachable!()
                }
            },
            0xF => {
                match lower {
                    0x07 => self.set_immediate(b2, self.delay_timer),
                    0x0A => self.get_key(b2),
                    0x15 => self.set_delay(b2),
                    0x18 => self.set_sound(b2),
                    0x1E => self.add_i(b2),
                    0x29 => self.set_i_sprite(b2),
                    0x33 => self.set_bcd(b2),
                    0x55 => self.reg_dump(b2),
                    0x65 => self.reg_load(b2),
                    _ => unreachable!()
                }
            },
            _ => unreachable!()
        }
    }

    fn sys(&mut self, value: u16) {
        // TODO: NOOP?
    }

    fn clear_screen(&mut self) {
        self.pixels.fill([false; 64]);
    }

    fn call(&mut self, value: u16) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = value as usize;
    }
    fn return_subr(&mut self) {
        // Decrement SP first to get back the original return PC
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    fn jmp(&mut self, value: u16) {
        self.pc = value as usize;
    }
    fn jmp_relative(&mut self, value: u16) {
        self.pc = value as usize + self.V[0] as usize;
    }

    // TODO: Could maybe simplify using an enum
    fn cond_check(&mut self, x: u8, (cmp_value, is_y): (u8, bool), equals: bool) {
        let vx = self.V[x as usize];
        let check = if is_y { self.V[cmp_value as usize] } else { cmp_value };

        if match equals {
            true => vx == check,
            false => vx != check
        } {
            self.pc += 2;
        }
    }

    fn set_immediate(&mut self, x: u8, nn: u8) {
        self.V[x as usize] = nn;
    }
    fn add_immediate(&mut self, x: u8, nn: u8) {
        let (res, _) = self.V[x as usize].overflowing_add(nn);
        self.V[x as usize] = res;
    }
    fn execute_arithmetic(&mut self, x: u8, y: u8, code: u8) {
        let x = x as usize;
        let y = y as usize;

        // TODO: Parameterize legacy mode
        let legacy = true;

        match code {
            0 => self.V[x] = self.V[y],
            1 => self.V[x] |= self.V[y],
            2 => self.V[x] &= self.V[y],
            3 => self.V[x] ^= self.V[y],
            4 => {
                let (res, overflow) = self.V[x].overflowing_add(self.V[y]);
                self.V[x] = res;
                self.V[0xF] = overflow as u8;
            },
            5 => {
                let (res, underflow) = self.V[x].overflowing_sub(self.V[y]);
                self.V[x] = res;
                self.V[0xF] = !underflow as u8;
            },
            6 => {
                let index = if legacy { y } else { x };
                let lsb = ((self.V[index] & 0x1) == 0x1) as u8;
                self.V[index] = self.V[index] >> 1;
                self.V[0xF] = lsb;
            },
            7 => {
                let (res, underflow) = self.V[y].overflowing_sub(self.V[x]);
                self.V[x] = res;
                self.V[0xF] = !underflow as u8;
            },
            0xE => {
                let index = if legacy { y } else { x };
                let msb = ((self.V[index] & 0x80) == 0x80) as u8;
                self.V[index] = self.V[index] << 1;
                self.V[0xF] = msb;
            },
            _ => unreachable!()
        }
    }
    fn set_random(&mut self, x: u8, nn: u8) {
        self.V[x as usize] = rand::random::<u8>() & nn;
    }

    fn set_i(&mut self, value: u16) {
        self.I = value;
    }
    fn add_i(&mut self, x: u8) {
        self.I = (self.I + self.V[x as usize] as u16); // TODO: ? & 0xFFF;
    }
    fn set_i_sprite(&mut self, x: u8) {
        // VX should be a single hex value (0-F)
        // Assuming fonts begin at 0x0, each font takes 5 bytes
        self.I = FONT_LOCATION + (self.V[x as usize] as u16 * 5);
    }
    fn set_bcd(&mut self, x: u8) {
        let vx = self.V[x as usize];

        self.memory[self.I as usize] = vx / 100;
        self.memory[self.I as usize + 1] = vx % 100 / 10;
        self.memory[self.I as usize + 2] = vx % 100 % 10;
    }

    fn draw(&mut self, x: u8, y: u8, n: u8) {
        let vx = self.V[x as usize] & 0x3F;
        let vy = self.V[y as usize] & 0x1F;

        self.V[0xF] = 0;

        // Behavior: the starting position should wrap (x & 0x3F, y & 0x1F)
        // But the drawing should NOT wrap
        for index in self.I..self.I + n as u16 {
            let mem_value = self.memory[index as usize];

            let y_offset = (index - self.I) as u8;
            for x_offset in 0..8 {
                let pixel_value = (1u8 << 7 - x_offset) & mem_value != 0;

                let y_index = (vy + y_offset) as usize;
                let x_index = (vx + x_offset) as usize;

                if let Some(row) = self.pixels.get_mut(y_index) {
                    if let Some(pixel) = row.get_mut(x_index) {
                        if *pixel && pixel_value {
                            self.V[0xF] = 1;
                        }

                        *pixel ^= pixel_value;
                    }
                }
            }
        }
    }

    // TODO: How to get key input?
    fn key_check(&mut self, x: u8, equals: bool) {}
    fn get_key(&mut self, x: u8) {}

    fn set_delay(&mut self, x: u8) {}
    fn set_sound(&mut self, x: u8) {}

    fn reg_dump(&mut self, x: u8) {
        for x_index in 0..=x {
            self.memory[self.I as usize] = self.V[x_index as usize];
            self.I += 1;
        }
        self.I += 1;
    }
    fn reg_load(&mut self, x: u8) {
        for x_index in 0..=x {
            self.V[x_index as usize] = self.memory[self.I as usize];
            self.I += 1;
        }
        self.I += 1;
    }
}