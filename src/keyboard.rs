pub struct Keyboard {
    pub pressed_key: Chip8Key,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            pressed_key: Chip8Key::None,
        }
    }

    #[inline]
    pub fn check_key(&mut self) -> Chip8Key {
        self.pressed_key
    }

    #[inline]
    pub fn press_key(&mut self, key: Chip8Key) {
        if key != Chip8Key::None {
            self.pressed_key = key;
        } else {
            self.pressed_key = Chip8Key::None;
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Chip8Key {
    Zero = 0x0,
    One = 0x1,
    Two = 0x2,
    Three = 0x3,
    Four = 0x4,
    Five = 0x5,
    Six = 0x6,
    Seven = 0x7,
    Eight = 0x8,
    Nine = 0x9,
    A = 0xA,
    B = 0xB,
    C = 0xC,
    D = 0xD,
    E = 0xE,
    F = 0xF,
    None,
}
