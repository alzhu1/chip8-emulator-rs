use sdl2::{event::Event, keyboard::Keycode, EventPump, Sdl};

use super::{Input, InputEvent, InputKey};

pub struct SDLInput {
    event_pump: EventPump,
}

impl SDLInput {
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn std::error::Error>> {
        let event_pump = sdl_context.event_pump()?;

        Ok(Self { event_pump })
    }
}

impl Input for SDLInput {
    // TODO: Seeing some dropped inputs for key release (and maybe key press)
    fn poll_input(&mut self) -> Option<InputEvent> {
        let mut input = None;

        for event in self.event_pump.poll_iter() {
            println!("Event: {:?}", event);

            input = match event {
                Event::Quit { .. } => Some(InputEvent::KeyPressed(InputKey::Quit)),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => Some(InputEvent::KeyPressed(InputKey::Quit)),
                    Keycode::Num0 => Some(InputEvent::KeyPressed(InputKey::K0)),
                    Keycode::Num1 => Some(InputEvent::KeyPressed(InputKey::K1)),
                    Keycode::Num2 => Some(InputEvent::KeyPressed(InputKey::K2)),
                    Keycode::Num3 => Some(InputEvent::KeyPressed(InputKey::K3)),
                    Keycode::Num4 => Some(InputEvent::KeyPressed(InputKey::K4)),
                    Keycode::Num5 => Some(InputEvent::KeyPressed(InputKey::K5)),
                    Keycode::Num6 => Some(InputEvent::KeyPressed(InputKey::K6)),
                    Keycode::Num7 => Some(InputEvent::KeyPressed(InputKey::K7)),
                    Keycode::Num8 => Some(InputEvent::KeyPressed(InputKey::K8)),
                    Keycode::Num9 => Some(InputEvent::KeyPressed(InputKey::K9)),
                    Keycode::A => Some(InputEvent::KeyPressed(InputKey::KA)),
                    Keycode::B => Some(InputEvent::KeyPressed(InputKey::KB)),
                    Keycode::C => Some(InputEvent::KeyPressed(InputKey::KC)),
                    Keycode::D => Some(InputEvent::KeyPressed(InputKey::KD)),
                    Keycode::E => Some(InputEvent::KeyPressed(InputKey::KE)),
                    Keycode::F => Some(InputEvent::KeyPressed(InputKey::KF)),
                    _ => None,
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Num0 => Some(InputEvent::KeyReleased(InputKey::K0)),
                    Keycode::Num1 => Some(InputEvent::KeyReleased(InputKey::K1)),
                    Keycode::Num2 => Some(InputEvent::KeyReleased(InputKey::K2)),
                    Keycode::Num3 => Some(InputEvent::KeyReleased(InputKey::K3)),
                    Keycode::Num4 => Some(InputEvent::KeyReleased(InputKey::K4)),
                    Keycode::Num5 => Some(InputEvent::KeyReleased(InputKey::K5)),
                    Keycode::Num6 => Some(InputEvent::KeyReleased(InputKey::K6)),
                    Keycode::Num7 => Some(InputEvent::KeyReleased(InputKey::K7)),
                    Keycode::Num8 => Some(InputEvent::KeyReleased(InputKey::K8)),
                    Keycode::Num9 => Some(InputEvent::KeyReleased(InputKey::K9)),
                    Keycode::A => Some(InputEvent::KeyReleased(InputKey::KA)),
                    Keycode::B => Some(InputEvent::KeyReleased(InputKey::KB)),
                    Keycode::C => Some(InputEvent::KeyReleased(InputKey::KC)),
                    Keycode::D => Some(InputEvent::KeyReleased(InputKey::KD)),
                    Keycode::E => Some(InputEvent::KeyReleased(InputKey::KE)),
                    Keycode::F => Some(InputEvent::KeyReleased(InputKey::KF)),
                    _ => None,
                },
                _ => break,
            };
        }

        input
    }
}
