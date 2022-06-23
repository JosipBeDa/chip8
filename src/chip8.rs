use crate::keyboard::*;
use crate::monitor::Monitor;
use rand::*;

const SPEED: u8 = 5;
const MEMORY_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16;
const SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pub monitor: Monitor,
    pub keyboard: Keyboard,
    memory: [u8; MEMORY_SIZE],
    registers: [u8; NUM_REGISTERS], // Vx where x = 0..F
    index: u16,                     // I
    pc: u16,
    stack: [u16; 16],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    speed: u8,
}

impl Chip8 {
    pub fn new(monitor: Monitor) -> Self {
        Self {
            monitor,
            memory: [0; MEMORY_SIZE],
            registers: [0; NUM_REGISTERS],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            speed: SPEED,
            keyboard: Keyboard::new(),
        }
    }

    pub fn get_metrics(&self) -> String {
        format!(
            "PC:{} I:{} SP:{} DT:{} ST:{} Key:{:?}",
            self.pc,
            self.index,
            self.stack_pointer,
            self.delay_timer,
            self.sound_timer,
            self.keyboard.pressed_key
        )
    }

    pub fn load_sprites(&mut self) {
        for (i, sprite_byte) in SPRITES.iter().enumerate() {
            self.memory[i] = *sprite_byte;
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (i, byte) in program.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn check_sound(&mut self) -> bool {
        self.sound_timer > 0
    }

    pub fn cycle(&mut self) {
        for _ in 0..self.speed {
            // Shift from the current location in memory 8 bits to the left,
            // e.g. 0xFF << 8 = 0xFF00
            let shifted: u16 = (self.memory[self.pc as usize] as u16) << 8;
            // Or it with the next instruction
            // e.g. 0xFF00 | 0x12 = 0xFF12
            let opcode: u16 = shifted | self.memory[self.pc as usize + 1] as u16;
            self.interpret_instruction(opcode);
            self.update_timers();
        }
    }

    #[allow(dead_code)]
    pub fn debug_cycle(&mut self) {
        for _ in 0..self.speed {
            let shifted: u16 = (self.memory[self.pc as usize] as u16) << 8;
            let opcode: u16 = shifted | self.memory[self.pc as usize + 1] as u16;
            self.debug_instruction(opcode);
            self.update_timers();
        }
    }

    pub fn interpret_instruction(&mut self, instruction: u16) {
        self.pc += 2;
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        match instruction & 0xF000 {
            0x0000 => match instruction {
                0x00E0 => {
                    self.monitor.clear();
                }
                0x00EE => {
                    self.pc = self.stack[self.stack_pointer as usize];
                    self.stack_pointer -= 1;
                }
                _ => {}
            },
            0x1000 => {
                self.pc = instruction & 0xFFF;
            }
            0x2000 => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.pc;
                self.pc = instruction & 0xFFF;
            }
            0x3000 => {
                if self.registers[x] == instruction as u8 {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.registers[x] != instruction as u8 {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            0x6000 => {
                self.registers[x] = instruction as u8;
            }
            0x7000 => {
                self.registers[x] = (self.registers[x] as u16 + (instruction & 0xFF)) as u8;
            }
            0x8000 => match instruction & 0xF {
                0x0 => self.registers[x] = self.registers[y],
                0x1 => self.registers[x] |= self.registers[y],
                0x2 => self.registers[x] &= self.registers[y],
                0x3 => self.registers[x] ^= self.registers[y],
                0x4 => {
                    self.registers[0xF] =
                        (self.registers[x] as u16 + self.registers[y] as u16 > 255) as u8;
                    self.registers[x] = (self.registers[x] as u16 + self.registers[y] as u16) as u8;
                }
                0x5 => {
                    self.registers[0xF] = (self.registers[x] > self.registers[y]) as u8;
                    self.registers[x] = (self.registers[x] as i16 - self.registers[y] as i16) as u8;
                }
                0x6 => {
                    self.registers[0xF] = self.registers[x] & 1;
                    self.registers[x] = self.registers[y] >> 1;
                }
                0x7 => {
                    self.registers[0xF] = (self.registers[x] < self.registers[y]) as u8;
                    self.registers[x] = (self.registers[y] as i16 - self.registers[x] as i16) as u8;
                }
                0xE => {
                    self.registers[0xF] = self.registers[x] & 0x80;
                    self.registers[x] = self.registers[y] << 1;
                }
                _ => {}
            },
            0x9000 => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.index = instruction & 0xFFF;
            }
            0xB000 => {
                self.pc += (instruction & 0xFFF) + self.registers[0] as u16;
            }
            0xC000 => {
                let rnd: u8 = thread_rng().gen_range(0..=255);
                self.registers[x] = rnd & instruction as u8;
            }
            0xD000 => {
                // Set carry flag to 0
                self.registers[0xF] = 0;
                // Fetch the y coordinate
                let mut c_y = self.registers[y] % 32;
                // The last nibble (n) will dictate how much bytes we read from the memory
                let n = instruction & 0xF;
                // Iterate through n bytes of memory
                for current_byte in 0..n {
                    // Fetch the x coordinate
                    let mut c_x = self.registers[x] % 64;
                    // Read the byte the index is pointing to incremented by the number of bytes already read
                    let mut sprite_byte = self.memory[(self.index + current_byte) as usize];
                    // Iterate through the bits of the sprite_byte
                    for _ in 0..8 {
                        // If the MSB of the sprite byte is 1 we set/unset the pixel and toggle the flag accordingly
                        if (sprite_byte & 0x80) > 0 {
                            // And if the pixel is togled to 1, set carry to 1
                            self.registers[0xF] =
                                self.monitor.toggle_pixel(c_x as usize, c_y as usize) as u8;
                        }
                        // Shift the byte 1 bit to the left so we can read the next bit
                        sprite_byte <<= 1;
                        c_x += 1;
                        if c_x > 63 {
                            break;
                        }
                    }
                    c_y += 1;
                    if c_y > 31 {
                        break;
                    }
                }
            }
            0xE000 => match instruction & 0xFF {
                0x9E => {
                    if let Some(key) = self.keyboard.check_key() {
                        if self.registers[x] == key as u8 {
                            self.pc += 2;
                        }
                    }
                }
                0xA1 => {
                    if let Some(key) = self.keyboard.check_key() {
                        if self.registers[x] != key as u8 {
                            self.pc += 2;
                        }
                    }
                }
                _ => {}
            },
            0xF000 => match instruction & 0xFF {
                0x07 => self.registers[x] = self.delay_timer,
                0x0A => {
                    match self.keyboard.check_key() {
                        Some(key) => {
                            self.registers[x] = key as u8;
                        }
                        None => {
                            self.pc -= 2;
                        }
                    };
                }
                0x15 => self.delay_timer = self.registers[x],
                0x18 => self.sound_timer = self.registers[x],
                0x1E => self.index += self.registers[x] as u16,
                0x29 => {
                    for i in 0..16 {
                        if self.registers[x] == i {
                            self.index = (i as usize * 5) as u16;
                            break;
                        }
                    }
                }
                0x33 => {
                    let mut digit = self.registers[x];
                    for i in 0..3 {
                        let m = digit;
                        self.memory[(self.index + 2 - i) as usize] = m % 10;
                        digit /= 10;
                    }
                }
                0x55 => {
                    for i in 0..=x {
                        self.memory[self.index as usize + i] = self.registers[i];
                    }
                    // self.index += 1 + x as u16;
                }
                0x65 => {
                    for i in 0..=x {
                        self.registers[i] = self.memory[self.index as usize + i];
                    }
                    // self.index += 1 + x as u16;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn debug_instruction(&mut self, instruction: u16) {
        println!("Interpreting instruction: {:#04x}", instruction);
        //std::thread::sleep(std::time::Duration::from_millis(500));
        self.pc += 2;
        let x = ((instruction & 0x0F00) >> 8) as usize;
        let y = ((instruction & 0x00F0) >> 4) as usize;
        match instruction & 0xF000 {
            0x0000 => match instruction {
                0x00E0 => {
                    println!("00E0 -- Clearing display");
                    self.monitor.clear();
                }
                0x00EE => {
                    self.pc = self.stack[self.stack_pointer as usize];
                    println!("00EE -- PC set to {:#4x}", self.pc);
                    self.stack_pointer -= 1;
                    println!("00EE -- Stack pointer set to {}", self.stack_pointer);
                }
                _ => {}
            },
            0x1000 => {
                self.pc = instruction & 0xFFF;
                println!("1NNN -- PC set to {:#4x}", self.pc);
            }
            0x2000 => {
                self.stack_pointer += 1;
                println!("2NNN -- Stack pointer set to {}", self.stack_pointer);
                self.stack[self.stack_pointer as usize] = self.pc;
                println!("2NNN -- Stack set to {:?}", self.stack);
                self.pc = instruction & 0xFFF;
                println!("2NNN -- PC set to {:#4x}", self.pc);
            }
            0x3000 => {
                if self.registers[x] == instruction as u8 {
                    self.pc += 2;
                    println!("3XNN -- PC set to {:#4x}", self.pc);
                }
            }
            0x4000 => {
                if self.registers[x] != instruction as u8 {
                    self.pc += 2;
                    println!("4XNN -- PC set to {:#4x}", self.pc);
                }
            }
            0x5000 => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                    println!("5XY0 -- PC set to {:#4x}", self.pc);
                }
            }
            0x6000 => {
                self.registers[x] = instruction as u8;
                println!("6XNN -- Register {} set to {:#4x}", x, instruction as u8);
            }
            0x7000 => {
                self.registers[x] = (self.registers[x] as u16 + (instruction & 0xFF)) as u8;
                println!("7XNN -- Register {} set to {:#4x}", x, self.registers[x]);
            }
            0x8000 => match instruction & 0xF {
                0x0 => {
                    self.registers[x] = self.registers[y];
                    println!("8XY0 -- Register {} set to {:#4x}", x, self.registers[x]);
                }
                0x1 => {
                    self.registers[x] |= self.registers[y];
                    println!("8XY1 -- Register {} set to {:#4x}", x, self.registers[x]);
                }
                0x2 => {
                    self.registers[x] &= self.registers[y];
                    println!("8XY2 -- Register {} set to {:#4x}", x, self.registers[x]);
                }
                0x3 => {
                    self.registers[x] ^= self.registers[y];
                    println!("8XY3 -- Register {} set to {:#4x}", x, self.registers[x]);
                }

                0x4 => {
                    self.registers[0xF] =
                        (self.registers[x] as u16 + self.registers[y] as u16 > 255) as u8;
                    println!("8XY4 -- Carry set to {}", self.registers[0xF]);
                    self.registers[x] = (self.registers[x] as u16 + self.registers[y] as u16) as u8;
                    println!("8XY4 -- Register {} set to {}", x, self.registers[x]);
                }

                0x5 => {
                    self.registers[0xF] = (self.registers[x] > self.registers[y]) as u8;
                    println!("8XY5 -- Carry set to {}", self.registers[0xF]);
                    self.registers[x] = (self.registers[x] as i16 - self.registers[y] as i16) as u8;
                    println!("8XY5 -- Register {} set to {}", x, self.registers[x]);
                }

                0x6 => {
                    //self.registers[x] = self.registers[y];
                    self.registers[0xF] = self.registers[x] & 1;
                    println!("8XY6 -- Carry set to {}", self.registers[0xF]);
                    self.registers[x] >>= 1;
                    println!("8XY6 -- Register {} set to {}", x, self.registers[x]);
                }

                0x7 => {
                    if self.registers[x] < self.registers[y] {
                        self.registers[0xF] = 1;
                    } else {
                        self.registers[0xF] = 0;
                    }
                    println!("8XY7 -- Carry set to {}", self.registers[0xF]);
                    self.registers[x] = match self.registers[y].checked_sub(self.registers[x]) {
                        Some(val) => val,
                        None => 0,
                    };
                    println!("8XY7 -- Register {} set to {}", x, self.registers[x]);
                }

                0xE => {
                    //self.registers[x] = self.registers[y];
                    self.registers[0xF] = self.registers[x] & 0x80;
                    self.registers[x] <<= 1;
                    println!("8XYE -- Register {} set to {}", x, self.registers[x]);
                }
                _ => {}
            },
            0x9000 => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                    println!("9XY0 -- PC set to {:#4x}", self.pc);
                }
            }
            0xA000 => {
                self.index = instruction & 0xFFF;
                println!("ANNN -- Index set to {}", self.index);
            }
            0xB000 => {
                self.pc += (instruction & 0xFFF) + self.registers[0] as u16;
                println!("BNNN -- PC set to {:#4x}", self.pc);
            }
            0xC000 => {
                let rnd: u8 = thread_rng().gen_range(0..=255);
                self.registers[x] = rnd & instruction as u8;
                println!("CXKK -- Register {} set to {}", x, self.registers[x]);
            }
            0xD000 => {
                let mut c_y = self.registers[y] % 32;
                self.registers[0xF] = 0;
                let n = instruction & 0xF;
                for current_byte in 0..n {
                    let mut c_x = self.registers[x] % 64;
                    let mut sprite_byte = self.memory[(self.index + current_byte) as usize];
                    for _ in 0..8 {
                        if (sprite_byte & 0x80) > 0 {
                            self.registers[0xF] =
                                self.monitor.toggle_pixel(c_x as usize, c_y as usize) as u8;
                        }
                        sprite_byte <<= 1;
                        c_x += 1;
                        if c_x > 63 {
                            break;
                        }
                    }
                    c_y += 1;
                    if c_y > 31 {
                        break;
                    }
                }
            }
            0xE000 => match instruction & 0xFF {
                0x9E => {
                    println!("0xEX9E -- checking for key");
                    if let Some(key) = self.keyboard.check_key() {
                        println!("EX9E -- Got key {:?}", key);
                        if self.registers[x] == key as u8 {
                            self.pc += 2;
                            println!("EX9E -- PC set to {:#4x}", self.pc);
                        }
                    }
                }
                0xA1 => {
                    println!("EXA1 -- checking for key");
                    if let Some(key) = self.keyboard.check_key() {
                        println!("EXA1 -- Got key {:?}", key);
                        if self.registers[x] != key as u8 {
                            self.pc += 2;
                            println!("EXA1 -- PC set to {:#4x}", self.pc);
                        }
                    }
                }
                _ => {}
            },
            0xF000 => match instruction & 0xFF {
                0x07 => {
                    self.registers[x] = self.delay_timer;
                    println!("FX07 -- Register {} set to {}", x, self.registers[x]);
                }

                0x0A => {
                    match self.keyboard.check_key() {
                        Some(key) => {
                            println!("FX0A -- Got key {:?}", key);
                            self.registers[x] = key as u8;
                            println!("FX0A -- Register {} set to {}", x, self.registers[x]);
                        }
                        None => {
                            self.pc -= 2;
                        }
                    };
                }

                0x15 => {
                    self.delay_timer = self.registers[x];
                    println!("FX15 -- Delay timer set to {}", self.delay_timer);
                }
                0x18 => {
                    self.sound_timer = self.registers[x];
                    println!("FX18 -- Sound timer set to {}", self.delay_timer);
                }

                0x1E => {
                    self.index += self.registers[x] as u16;
                    println!("FX1E -- Index set to {}", self.index);
                }

                0x29 => {
                    for i in 0..16 {
                        if self.registers[x] == i {
                            self.index = (i as usize * 5) as u16;
                            println!("FX29 -- Index set to {}", self.index);
                            break;
                        }
                    }
                }

                0x33 => {
                    let mut digit = self.registers[x];
                    for i in 0..3 {
                        let m = digit;
                        self.memory[(self.index + 2 - i) as usize] = m % 10;
                        digit /= 10;
                        println!(
                            "FX33 -- Memory location {} set to {}",
                            self.index + 2 - i,
                            self.memory[(self.index + 2 - i) as usize]
                        );
                    }
                }

                0x55 => {
                    for i in 0..=x {
                        self.memory[self.index as usize + i] = self.registers[i];
                    }
                    // self.index += 1 + x as u16;
                }

                0x65 => {
                    for i in 0..=x {
                        self.registers[i] = self.memory[self.index as usize + i];
                        println!("FX65 -- Register {} set to {}", i, self.registers[i]);
                    }
                    // self.index += 1 + x as u16;
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn carry_sanity() {
        let x: u8 = 12;
        let y: u8 = 254;
        let c = (x as u16 + y as u16) as u8;
        let d = (x as i8 - y as i8) as u8;
        assert_eq!(x as i8, 12);
        assert_eq!(c, 10);
        assert_eq!(d, 14);
    }
    use super::*;
    #[test]
    fn bool_sanity() {
        assert_eq!(true as u8, 1);
        assert_eq!(false as u8, 0);
    }

    #[test]
    fn shift_sanity() {
        let i: u8 = 0xFF;
        let shifted: u16 = (i as u16) << 8;
        assert_eq!(shifted, 0xFF00);
    }

    #[test]
    fn digit_test() {
        let mut sample = [0u8; 3];
        let mut sample2 = [0u8; 3];
        let mut digit: u8 = 12;
        let mut digit2: u8 = 237;
        for i in 0..3 {
            let a = digit;
            let b = digit2;
            sample[2 - i] = a % 10;
            sample2[2 - i] = b % 10;
            digit /= 10;
            digit2 /= 10;
        }
        assert_eq!(digit, 0);
        assert_eq!(sample, [0 as u8, 1 as u8, 2 as u8]);
        assert_eq!(sample2, [2 as u8, 3 as u8, 7 as u8]);
    }

    #[test]
    fn reg_mem_test() {
        let mut memory = [0; MEMORY_SIZE];
        let mut registers = [0u8; 16];
        let mut rng = thread_rng();

        // Fill registers 2 through 7 with random values
        for i in 2..8 {
            let rnd: u8 = rng.gen_range(0..=255);
            registers[i] = rnd;
        }

        let x = 5;
        let mut index = 2;
        // Start from register 0, write to memory
        for i in 0..=x {
            memory[index as usize + i] = registers[i];
        }
        index += 1 + x as u16;

        assert_eq!(memory[2..7], registers[0..5]);
        assert_eq!(index, 8);

        let x = 3;
        for i in 0..=x {
            registers[i] = memory[index as usize + i];
        }
        index += 1 + x as u16;

        assert_eq!(memory[8..11], registers[0..3]);
        assert_eq!(index, 12);
    }

    #[test]
    fn drawing() {
        // Mock parts
        let mut monitor = Monitor::new_default();
        let mut registers = [0u8; 16];
        let mut memory = [0u8; 4096];
        let index = 0;

        // Mock instruction
        let instruction = 0xD7C5;
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        assert_eq!(x, 7);
        assert_eq!(y, 12);

        // Set values
        registers[x] = 234;
        registers[y] = 67;

        // Load the sprite 0 into memory
        let zero = [0xF0, 0x90, 0x90, 0x90, 0xF0];
        for i in 0..5 {
            memory[index + i] = zero[i];
        }

        let mut c_x = registers[x] % 64;
        let mut c_y = registers[y] % 32;
        registers[0xF] = 0;
        assert_eq!(c_x, 42);
        assert_eq!(c_y, 3);

        // The last nibble (n) will dictate how much bytes we read from the memory
        let n = instruction & 0xF;
        assert_eq!(n, 5);

        // Iterate through n bytes of the memory
        for current_byte in 0..n {
            // Load sprite bytes - Read the byte the index is pointing to incremented by the number of bytes already read
            let mut sprite_byte = memory[(index + current_byte) as usize];

            // Iterate through the bits of the sprite_byte
            for _ in 0..8 {
                // If the MSB of the sprite byte is 1 we set/unset the pixel and toggle the flag accordingly
                if sprite_byte & 0x80 > 0 {
                    if monitor.toggle_pixel(c_x as usize, c_y as usize) {
                        registers[0xF] = 1;
                    }
                }
                // Shift the byte 1 bit to the left so we can read the next bit
                sprite_byte <<= 1;
                c_x += 1;
                if c_x > 63 {
                    break;
                }
            }
            c_y += 1;
            if c_y > 31 {
                break;
            }
        }
        assert_eq!(monitor.get_buffer()[234], 1);
    }
}
