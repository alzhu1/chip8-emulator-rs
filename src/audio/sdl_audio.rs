use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    Sdl,
};

use super::Audio;

// BEGIN EXAMPLE TODO: MOVE THIS

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

// END EXAMPLE

// TODO: Make the audio callback a generic trait bound
pub struct SDLAudio {
    device: AudioDevice<SquareWave>,
}

impl SDLAudio {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn std::error::Error>> {
        let audio_subsystem = sdl_context.audio()?;

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        Ok(Self { device })
    }
}

impl Audio for SDLAudio {
    fn pause_audio(&mut self) {
        self.device.pause();
    }

    fn resume_audio(&mut self) {
        self.device.resume();
    }
}
