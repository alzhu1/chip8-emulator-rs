use chip8_emulator_rs::{
    audio::{sdl_audio::SDLAudio, Audio},
    cpu::CPU,
    input::{sdl_input::SDLInput, Input, InputEvent, InputKey},
    video::{sdl_video::SDLVideo, Video},
    SCREEN_WIDTH
};

const VIDEO_WIDTH: usize = SCREEN_WIDTH * 16;

use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;

    // Init CPU
    let mut cpu = CPU::new(chip8_emulator_rs::cpu::CPUVariant::SChipv1_1);

    // Init audio/input drivers
    let mut sdl_audio = SDLAudio::new(&sdl_context)?;
    let mut sdl_input = SDLInput::new(&sdl_context)?;

    // Init video driver, use CPU resolution as basis
    let scale = (VIDEO_WIDTH / cpu.max_res.0) as u32;
    let mut sdl_video = SDLVideo::new(&sdl_context, scale, cpu.max_res.0, cpu.max_res.1)?;

    let frame_ms = Duration::from_nanos(16_666_666);

    let args = std::env::args();
    let rom = match args.len() {
        2 => args.last().unwrap(),
        _ => String::from("test_rom.ch8"),
    };

    cpu.load_rom(rom);

    loop {
        let frame_start_time = Instant::now();
        if let Some(input) = sdl_input.poll_input() {
            match input {
                InputEvent::KeyPressed(key) => match key {
                    // Break if we quit
                    InputKey::Quit => break,
                    _ => cpu.press_key(key as u8),
                },
                InputEvent::KeyReleased(key) => match key {
                    InputKey::Quit => (),
                    _ => cpu.release_key(key as u8),
                },
            }
        }

        // Process CPU instructions
        for _ in 0..20 {
            cpu.process();

            if cpu.should_vblank() {
                break;
            }
        }

        cpu.decrement_timers();

        if cpu.is_sound_active() {
            sdl_audio.resume_audio();
        } else {
            sdl_audio.pause_audio();
        }

        cpu.reset_vblank();
        sdl_video.draw_to_window(&cpu.pixels, cpu.max_res.0, cpu.max_res.1);

        // println!("Process + draw time: {}ms", a.elapsed().as_millis());

        // TODO: Need to figure out timing
        // Thinking that we could do the following:
        /*
         * The main loop runs at a consistent frame rate (i.e. 60 FPS)
         * We process CPU instructions, and calculate the amount of instructions that should be processed
         * Use https://jackson-s.me/2019/07/13/Chip-8-Instruction-Scheduling-and-Frequency.html as ref
         * (Alternative can just aim for CPU rate of ~500hz)
         *
         * If the CPU has processed enough, check if we need to draw to the screen
         *
         *
         */

        let remaining_time = frame_ms.saturating_sub(frame_start_time.elapsed());

        if !remaining_time.is_zero() {
            std::thread::sleep(remaining_time);
        }

        // println!("Elapsed time for frame: {}ms", global_timer.elapsed().as_millis());
    }

    Ok(())
}
