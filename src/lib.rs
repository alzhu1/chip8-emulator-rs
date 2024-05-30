// Constants
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub const MAX_RESOLUTION_WIDTH: usize = 256;
pub const MAX_RESOLUTION_HEIGHT: usize = 196;

pub mod cpu;

// Modules for other parts of emulator
pub mod audio;
pub mod input;
pub mod video;
