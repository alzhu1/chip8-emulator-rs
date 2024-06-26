pub mod sdl_video;

pub trait Video {
    fn draw_to_window<'a, I, J>(&mut self, pixels: I, width: usize, height: usize)
    where
        I: IntoIterator<Item = J>,
        J: IntoIterator<Item = &'a bool>;
}
