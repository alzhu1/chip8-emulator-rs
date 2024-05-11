mod cpu;

// // TODO: To support CHIP-8 variants, could introduce an enum to Chip8 struct

// // TODO: I think this whole thing is the CPU itself? So loop should be done outside of this?
// // Or the loop should be done in a Chip8 APP? Or this is renamed as CPU?

// // Entry point to chip8 emulator
// pub struct Chip8 {
//     memory: [u8; 4096],

//     V: [u8; 0xF], // registers
//     I: u16,       // 12-bit index reg
//     pc: u16,
//     delay_timer: u8,
//     sound_timer: u8,

//     // TODO: Is this the best place to put this?
//     stack: [u16; 16],
//     sp: u16,

//     // TODO: Array or bitmask?
//     key: u16
// }

// impl Default for Chip8 {
//     fn default() -> Self {
//         Self {
//             memory: [0; 4096],
//             V: [0; 0xF],
//             I: 0,
//             // TODO: Some programs can start elsewhere
//             pc: 0x200,
//             delay_timer: 0,
//             sound_timer: 0,
//             stack: [0; 16],
//             sp: 0,
//             key: 0
//         }
//     }
// }

// impl Chip8 {
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