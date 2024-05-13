pub mod sdl_input;

pub enum InputKey {
    K0 = 0x0,
    K1 = 0x1,
    K2 = 0x2,
    K3 = 0x3,
    K4 = 0x4,
    K5 = 0x5,
    K6 = 0x6,
    K7 = 0x7,
    K8 = 0x8,
    K9 = 0x9,
    KA = 0xA,
    KB = 0xB,
    KC = 0xC,
    KD = 0xD,
    KE = 0xE,
    KF = 0xF,
    Quit = 0x80,
}

pub enum InputEvent {
    KeyPressed(InputKey),
    KeyReleased(InputKey),
}

pub trait Input {
    fn poll_input(&mut self) -> Option<InputEvent>;
}
