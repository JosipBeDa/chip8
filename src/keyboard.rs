pub struct Keyboard {
    pressed_key: Option<Chip8Key>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { pressed_key: None }
    }
    pub fn check_key(&mut self) -> Option<Chip8Key> {
        println!("Checking key, got: {:?}", self.pressed_key);
        match &self.pressed_key {
            Some(key) => {
                let pressed_key = *key;
                self.pressed_key = None;
                Some(pressed_key)
            }
            None => None,
        }
    }
    pub fn press_key(&mut self, key: Option<Chip8Key>) {
        if let Some(key) = key {
            self.pressed_key = Some(key);
        } else {
            self.pressed_key = None;
        }
        println!("Pressing  key {:?}", self.pressed_key);
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
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
}
