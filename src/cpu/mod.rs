#![allow(non_snake_case)]

use std::{fs::File, io::Read};

mod config;

use crate::{MAX_RESOLUTION_HEIGHT, MAX_RESOLUTION_WIDTH};

use self::config::CPUConfig;

// Export from CPU module
pub use config::CPUVariant;

const FONT_LOCATION: usize = 0x0;
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

enum PollingKeyPress {
    Polling(usize),
    NotPolling,
}

enum CondCheck {
    NN(u8),
    VY(usize),
}

// Entry point to chip8 emulator
pub struct CPU {
    config: CPUConfig, // Config, for quirks/variant
    pub pixels: [[bool; MAX_RESOLUTION_WIDTH]; MAX_RESOLUTION_HEIGHT], // Pixel memory
    memory: [u8; 4096], // RAM
    V: [u8; 0x10],     // V registers
    I: usize,          // 12-bit index reg
    pc: usize,         // Program counter
    delay_timer: u8,   // Delay timer
    sound_timer: u8,   // Sound timer
    stack: [usize; 16], // Stack for return addr
    sp: usize,         // Stack pointer
    keys: u16,         // Keys pressed
    polling_key_press: PollingKeyPress, // Check polling

    curr_res: (usize, usize),
    pub max_res: (usize, usize),
}

impl CPU {
    pub fn new(variant: CPUVariant) -> Self {
        let mut memory = [0; 4096];
        for (i, &font_byte) in FONT_BYTES.iter().enumerate() {
            memory[FONT_LOCATION + i] = font_byte;
        }

        let config = CPUConfig::from(variant);

        // TODO: Notes on resolution
        // Idea here is that we should default to hires if it exists

        // If we're in a hires mode, modify pixels as normal
        // If we're in a lores mode, pixel location * 2 = top left corner? Paint 2x2

        // I think it makes sense to take inspiration from Cadmium
        // Basically, the buffer should just be the max size possible (i.e. Megachip)
        // Not sure how this part is in Cadmium, but basically:

        // We limit ourselves to the top left corner of the buffer
        // If no hires supported, use the base resolution
        // If hires supported, use the higher resolution, and lores draws 2x2
        // Any excess parts of the screen should get chopped off
        // let (width, height) = if let Some(hires_resolution) = config.hires_resolution {
        //     hires_resolution
        // } else {
        //     config.base_resolution
        // };

        // let pixels = vec![vec![false; width]; height];
        let curr_res = config.resolutions[0];
        let max_res = *config.resolutions.last().unwrap();

        Self {
            config,
            pixels: [[false; MAX_RESOLUTION_WIDTH]; MAX_RESOLUTION_HEIGHT],
            memory,
            V: [0; 0x10],
            I: 0,
            // TODO: Some programs can start elsewhere
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keys: 0,
            polling_key_press: PollingKeyPress::NotPolling,
            curr_res,
            max_res,
        }
    }

    pub fn load_rom(&mut self, rom: String) {
        let mut file = File::open(rom).unwrap();

        // TODO: Clippy complains about partial read
        file.read(&mut self.memory[0x200..]).unwrap();
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn is_sound_active(&self) -> bool {
        self.sound_timer > 0
    }

    pub fn press_key(&mut self, key_index: u8) {
        self.keys |= 1u16 << key_index;
    }

    pub fn release_key(&mut self, key_index: u8) {
        self.keys &= !(1u16 << key_index);

        if let PollingKeyPress::Polling(x) = self.polling_key_press {
            self.V[x] = key_index;
            self.polling_key_press = PollingKeyPress::NotPolling;
        }
    }

    pub fn process(&mut self, drawing: &mut bool) {
        if let PollingKeyPress::Polling(_) = self.polling_key_press {
            return;
        }

        let upper = self.memory[self.pc];
        let lower = self.memory[self.pc + 1];
        let instruction = ((upper as u16) << 8) | (lower as u16);

        let (b1, b2) = (upper >> 4, upper & 0xF);
        let (b3, b4) = (lower >> 4, lower & 0xF);

        // Increment here, so that jumps aren't affected
        self.pc += 2;

        // Helper vars
        let nnn = (instruction & 0xFFF) as usize;
        let nn = lower;
        let x = b2 as usize;
        let y = b3 as usize;
        let n = b4 as usize;

        match b1 {
            0 => match lower {
                _ if b3 == 0xC => self.scroll_down(n), // 00CN
                0xE0 => self.clear_screen(),
                0xEE => self.return_subr(),
                0xFB => self.scroll_right(),
                0xFC => self.scroll_left(),
                0xFE => self.set_hires(false),
                0xFF => self.set_hires(true),
                _ => self.sys(nnn),
            },
            1 => self.jmp(nnn),
            2 => self.call(nnn),
            3 => self.cond_check(x, CondCheck::NN(nn), true),
            4 => self.cond_check(x, CondCheck::NN(nn), false),
            5 => match b4 {
                0 => self.cond_check(x, CondCheck::VY(y), true),
                _ => unreachable!(),
            },
            6 => self.set_immediate(x, nn),
            7 => self.add_immediate(x, nn),
            8 => self.execute_arithmetic(x, y, b4),
            9 => self.cond_check(x, CondCheck::VY(y), false),
            0xA => self.set_i(nnn),
            0xB => self.jmp_relative(nnn),
            0xC => self.set_random(x, nn),
            0xD => self.draw(x, y, n, drawing),
            0xE => match lower {
                0x9E => self.key_check(x, true),
                0xA1 => self.key_check(x, false),
                _ => unreachable!(),
            },
            0xF => match lower {
                0x07 => self.set_immediate(x, self.delay_timer),
                0x0A => self.get_key(x),
                0x15 => self.set_delay(x),
                0x18 => self.set_sound(x),
                0x1E => self.add_i(x),
                0x29 => self.set_i_sprite(x),
                0x33 => self.set_bcd(x),
                0x55 => self.reg_dump(x),
                0x65 => self.reg_load(x),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    fn sys(&mut self, _nnn: usize) {
        // TODO: NOOP?
    }

    fn clear_screen(&mut self) {
        for row in &mut self.pixels {
            row.fill(false);
        }
    }

    // Scrolling
    // TODO: Need to set a configuration value for modern/legacy
    fn scroll_down(&mut self, n: usize) {
        let n = n * (self.max_res.1 / self.curr_res.1);
        for r in (0..self.max_res.1).rev() {
            match r >= n {
                true => {
                    let slice = self.pixels[r - n];
                    self.pixels[r].clone_from_slice(&slice);
                }
                false => self.pixels[r].fill(false),
            }
        }
    }

    fn scroll_right(&mut self) {
        let scroll_amount = 4 * (self.max_res.0 / self.curr_res.0);

        for row in &mut self.pixels {
            for p in (0..self.max_res.0).rev() {
                match p < scroll_amount {
                    true => row[p] = false,
                    false => row[p] = row[p - scroll_amount],
                }
            }
        }
    }

    fn scroll_left(&mut self) {
        let scroll_amount = 4 * (self.max_res.0 / self.curr_res.0);

        for row in &mut self.pixels {
            for p in 0..self.max_res.0 {
                match p >= self.max_res.0 - scroll_amount {
                    true => row[p] = false,
                    false => row[p] = row[p + scroll_amount],
                }
            }
        }
    }

    fn call(&mut self, nnn: usize) {
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }
    fn return_subr(&mut self) {
        // Decrement SP first to get back the original return PC
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    // Hires
    fn set_hires(&mut self, is_hires: bool) {
        // TODO: Gate this behind variant?
        // self.is_hires = is_hires;
        self.curr_res = match is_hires {
            true => *self.config.resolutions.last().unwrap(),
            false => *self.config.resolutions.first().unwrap(),
        };

        // TODO: Clear screen for now
        self.clear_screen();
    }

    fn jmp(&mut self, nnn: usize) {
        self.pc = nnn;
    }
    fn jmp_relative(&mut self, nnn: usize) {
        self.pc = match self.config.jump_quirk {
            true => nnn + self.V[(nnn >> 8) & 0xF] as usize,
            false => nnn + self.V[0] as usize,
        }
    }

    fn cond_check(&mut self, x: usize, check: CondCheck, equals: bool) {
        let vx = self.V[x];
        let check = match check {
            CondCheck::NN(nn) => nn,
            CondCheck::VY(y) => self.V[y],
        };

        if match equals {
            true => vx == check,
            false => vx != check,
        } {
            self.pc += 2;
        }
    }

    fn set_immediate(&mut self, x: usize, nn: u8) {
        self.V[x] = nn;
    }
    fn add_immediate(&mut self, x: usize, nn: u8) {
        self.V[x] = self.V[x].overflowing_add(nn).0;
    }

    fn execute_arithmetic(&mut self, x: usize, y: usize, code: u8) {
        match code {
            0 => self.V[x] = self.V[y],
            1 => {
                self.V[x] |= self.V[y];
                if self.config.logic_quirk {
                    self.V[0xF] = 0;
                }
            }
            2 => {
                self.V[x] &= self.V[y];
                if self.config.logic_quirk {
                    self.V[0xF] = 0;
                }
            }
            3 => {
                self.V[x] ^= self.V[y];
                if self.config.logic_quirk {
                    self.V[0xF] = 0;
                }
            }
            4 => {
                let (res, overflow) = self.V[x].overflowing_add(self.V[y]);
                self.V[x] = res;
                self.V[0xF] = overflow as u8;
            }
            5 => {
                let (res, underflow) = self.V[x].overflowing_sub(self.V[y]);
                self.V[x] = res;
                self.V[0xF] = !underflow as u8;
            }
            6 => {
                let index = if self.config.shift_quirk { x } else { y };
                let lsb = ((self.V[index] & 0x1) == 0x1) as u8;
                self.V[x] = self.V[index] >> 1;
                self.V[0xF] = lsb;
            }
            7 => {
                let (res, underflow) = self.V[y].overflowing_sub(self.V[x]);
                self.V[x] = res;
                self.V[0xF] = !underflow as u8;
            }
            0xE => {
                let index = if self.config.shift_quirk { x } else { y };
                let msb = ((self.V[index] & 0x80) == 0x80) as u8;
                self.V[x] = self.V[index] << 1;
                self.V[0xF] = msb;
            }
            _ => unreachable!(),
        }
    }
    fn set_random(&mut self, x: usize, nn: u8) {
        self.V[x] = rand::random::<u8>() & nn;
    }

    fn set_i(&mut self, nnn: usize) {
        self.I = nnn;
    }
    fn add_i(&mut self, x: usize) {
        self.I += self.V[x] as usize; // TODO: ? & 0xFFF;
    }
    fn set_i_sprite(&mut self, x: usize) {
        // VX should be a single hex value (0-F)
        // Assuming fonts begin at 0x0, each font takes 5 bytes
        self.I = FONT_LOCATION + (self.V[x] as usize * 5);
    }
    fn set_bcd(&mut self, x: usize) {
        let vx = self.V[x];

        self.memory[self.I] = vx / 100;
        self.memory[self.I + 1] = vx % 100 / 10;
        self.memory[self.I + 2] = vx % 100 % 10;
    }

    fn draw(&mut self, x: usize, y: usize, n: usize, drawing: &mut bool) {
        *drawing = true;

        // Slightly counter-intuitive, but we should mod by curr res
        // This is because VX and VY are memory indices
        // They shouldn't be affected by varying size
        let vx = (self.V[x] as usize) % self.curr_res.0;
        let vy = (self.V[y] as usize) % self.curr_res.1;

        let x_size = self.max_res.0 / self.curr_res.0;
        let y_size = self.max_res.1 / self.curr_res.1;

        self.V[0xF] = 0;

        // Behavior: the starting position should wrap (x & currWidth, y & currHeight)
        // But the drawing should NOT wrap
        for index in self.I..self.I + n {
            let mem_value = self.memory[index];

            let y_offset = index - self.I;
            for x_offset in 0..8 {
                let pixel_value = (1u8 << (7 - x_offset)) & mem_value != 0;

                // Multiply by size to get correct offsets into pixel buffer
                let y_index = (vy + y_offset) * y_size;
                let x_index = (vx + x_offset) * x_size;

                // Draw pixels
                for y_index in y_index..y_index + y_size {
                    for x_index in x_index..x_index + x_size {
                        if x_index < self.max_res.0 && y_index < self.max_res.1 {
                            if let Some(pixel) = self
                                .pixels
                                .get_mut(y_index)
                                .and_then(|row| row.get_mut(x_index))
                            {
                                if *pixel && pixel_value {
                                    self.V[0xF] = 1;
                                }

                                *pixel ^= pixel_value;
                            }
                        }
                    }
                }
            }
        }
    }

    fn key_check(&mut self, x: usize, equals: bool) {
        let vx = self.V[x];
        // println!("Checking input, x = {}, vx = {}, key state = {:#x}", x, vx, self.key);
        if match equals {
            true => (1u16 << vx) & self.keys != 0,
            false => (1u16 << vx) & self.keys == 0,
        } {
            self.pc += 2;
        }
    }
    fn get_key(&mut self, x: usize) {
        self.polling_key_press = PollingKeyPress::Polling(x);
    }

    fn set_delay(&mut self, x: usize) {
        self.delay_timer = self.V[x];
    }
    fn set_sound(&mut self, x: usize) {
        self.sound_timer = self.V[x];
    }

    fn reg_dump(&mut self, x: usize) {
        let (quirk, offset) = self.config.load_store_quirk_offset;

        for x_index in 0..=x {
            self.memory[self.I + x_index] = self.V[x_index];
        }

        if !quirk {
            self.I += x;
            self.I += offset;
        }
    }
    fn reg_load(&mut self, x: usize) {
        let (quirk, offset) = self.config.load_store_quirk_offset;

        for x_index in 0..=x {
            self.V[x_index] = self.memory[self.I + x_index];
        }

        if !quirk {
            self.I += x;
            self.I += offset;
        }
    }
}
