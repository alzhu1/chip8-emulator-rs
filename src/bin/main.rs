// use chip8::Chip8;
use chip8::cpu::CPU;

use std::time::{Duration, Instant};

use sdl2::{audio::{AudioCallback, AudioSpecDesired}, event::Event, keyboard::Keycode, pixels::Color, rect::{Point, Rect}};

// BEGIN EXAMPLE TODO: MOVE THIS

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;
    
    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };
    
    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25
        }
    }).unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    let mut cpu = CPU::default();
    let mut auto = true;

    let mut global_timer = Instant::now();

    let frame_ms = Duration::from_nanos(16_666_666);

    'running: loop {
        // TODO: Move event polling to a separate module
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    cpu.process();
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    auto = !auto;
                },
                Event::KeyDown { keycode, .. } => {
                    match keycode.unwrap() {
                        Keycode::Num0 => cpu.press_key(0),
                        Keycode::Num1 => cpu.press_key(1),
                        Keycode::Num2 => cpu.press_key(2),
                        Keycode::Num3 => cpu.press_key(3),
                        Keycode::Num4 => cpu.press_key(4),
                        Keycode::Num5 => cpu.press_key(5),
                        Keycode::Num6 => cpu.press_key(6),
                        Keycode::Num7 => cpu.press_key(7),
                        Keycode::Num8 => cpu.press_key(8),
                        Keycode::Num9 => cpu.press_key(9),
                        Keycode::A => cpu.press_key(0xA),
                        Keycode::B => cpu.press_key(0xB),
                        Keycode::C => cpu.press_key(0xC),
                        Keycode::D => cpu.press_key(0xD),
                        Keycode::E => cpu.press_key(0xE),
                        Keycode::F => cpu.press_key(0xF),
                        _ => ()
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    match keycode.unwrap() {
                        Keycode::Num0 => cpu.release_key(0),
                        Keycode::Num1 => cpu.release_key(1),
                        Keycode::Num2 => cpu.release_key(2),
                        Keycode::Num3 => cpu.release_key(3),
                        Keycode::Num4 => cpu.release_key(4),
                        Keycode::Num5 => cpu.release_key(5),
                        Keycode::Num6 => cpu.release_key(6),
                        Keycode::Num7 => cpu.release_key(7),
                        Keycode::Num8 => cpu.release_key(8),
                        Keycode::Num9 => cpu.release_key(9),
                        Keycode::A => cpu.release_key(0xA),
                        Keycode::B => cpu.release_key(0xB),
                        Keycode::C => cpu.release_key(0xC),
                        Keycode::D => cpu.release_key(0xD),
                        Keycode::E => cpu.release_key(0xE),
                        Keycode::F => cpu.release_key(0xF),
                        _ => ()
                    }
                }
                _ => {}
            }
        }

        // Process 8 CPU instructions
        let a = Instant::now();
        for _ in 0..20 {
            cpu.process();

            if cpu.drawing {
                break;
            }
        }

        cpu.decrement_timers();

        if cpu.sound_timer == 0 {
            device.pause();
        } else {
            device.resume();
        }

        let b = Instant::now();

        if cpu.drawing {
            cpu.drawing = false;

            // TODO: Move drawing to another module
            canvas.clear();
            let rects = cpu.pixels.iter().enumerate().flat_map(|(y, row)| {
                row.iter().enumerate().map(move |(x, pixel)| {
                    let rect = Rect::new(x as i32 * 10, y as i32 * 10, 10, 10);
                    match *pixel {
                        true => [Some(rect), None],
                        false => [None, Some(rect)]
                    }
                })
            }).collect::<Vec<[Option<Rect>; 2]>>();
            canvas.set_draw_color(Color::WHITE);
            canvas.fill_rects(rects.iter().filter(|r| r[0].is_some()).map(|r| r[0].unwrap()).collect::<Vec<Rect>>().as_slice())?;
            canvas.set_draw_color(Color::BLACK);
            canvas.fill_rects(rects.iter().filter(|r| r[1].is_some()).map(|r| r[1].unwrap()).collect::<Vec<Rect>>().as_slice())?;
            canvas.present();
        }

        // println!("Draw time: {}ms", b.elapsed().as_millis());

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

        let remaining_time = frame_ms.saturating_sub(a.elapsed());

        if !remaining_time.is_zero() {
            std::thread::sleep(remaining_time);
        }

        // println!("Elapsed time for frame: {}ms", global_timer.elapsed().as_millis());
        global_timer = Instant::now();
    }

    Ok(())
}
