use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

use super::Video;

pub struct SDLVideo {
    canvas: Canvas<Window>,
    scale: u32,
}

impl SDLVideo {
    pub fn new(sdl_context: &Sdl, scale: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let video_subsystem = sdl_context.video()?;

        let window_width = SCREEN_WIDTH as u32 * scale;
        let window_height = SCREEN_HEIGHT as u32 * scale;

        let window = video_subsystem
            .window("TODO: BETTER NAME", window_width, window_height)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;

        Ok(Self { canvas, scale })
    }
}

impl Video for SDLVideo {
    fn draw_to_window(&mut self, pixels: &[[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let scale = self.scale;

        let rects = pixels
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, pixel)| {
                    let rect = Rect::new(
                        x as i32 * scale as i32,
                        y as i32 * scale as i32,
                        scale,
                        scale,
                    );
                    match *pixel {
                        true => Some(rect),
                        false => None,
                    }
                })
            })
            .collect::<Vec<Rect>>();

        // Draw pixels
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.fill_rects(rects.as_slice()).unwrap();

        // Update screen
        self.canvas.present();
    }
}
