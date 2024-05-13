pub mod sdl_audio;

pub trait Audio {
    fn resume_audio(&mut self);
    fn pause_audio(&mut self);
}
