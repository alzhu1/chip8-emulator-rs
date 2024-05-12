// use chip8::Chip8;
use chip8::cpu::CPU;

use std::time::Duration;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::{Point, Rect}};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    let mut cpu = CPU::default();
    let mut auto = false;

    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    cpu.process();
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    auto = !auto;
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    cpu.memory[0x1FF] = 1;
                },
                _ => {}
            }
        }

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

        // let k = rects.iter().filter(|r| r[0].is_some()).map(|r| r[0].unwrap()).collect::<Vec<Rect>>();

        canvas.fill_rects(rects.iter().filter(|r| r[0].is_some()).map(|r| r[0].unwrap()).collect::<Vec<Rect>>().as_slice())?;
        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rects(rects.iter().filter(|r| r[1].is_some()).map(|r| r[1].unwrap()).collect::<Vec<Rect>>().as_slice())?;

        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        if auto {
            cpu.process();
        }
    }

    Ok(())
}
