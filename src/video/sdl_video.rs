use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};

use super::Video;

pub struct SDLVideo {
    canvas: Canvas<Window>,
    scale: u32,
}

impl SDLVideo {
    pub fn new(
        sdl_context: &Sdl,
        scale: u32,
        width: usize,
        height: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let video_subsystem = sdl_context.video()?;

        let window_width = width as u32 * scale;
        let window_height = height as u32 * scale;

        let window = video_subsystem
            .window("TODO: BETTER NAME", window_width, window_height)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;

        Ok(Self { canvas, scale })
    }
}

impl Video for SDLVideo {
    fn draw_to_window<'a, I, J>(&mut self, pixels: I, width: usize, height: usize)
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = &'a bool>,
    {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let scale = self.scale;

        let rects = pixels
            .into_iter()
            .take(height)
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter().take(width).enumerate().filter_map(move |(x, pixel)| {
                    let rect = Rect::new(
                        x as i32 * scale as i32,
                        y as i32 * scale as i32,
                        scale,
                        scale,
                    );
                    match pixel {
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
