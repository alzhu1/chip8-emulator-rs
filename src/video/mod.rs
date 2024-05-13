use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub mod sdl_video;

pub trait Video {
    fn draw_to_window(&mut self, pixels: &[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]);
}
