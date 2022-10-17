pub const COLS: usize = 64;
pub const ROWS: usize = 32;
pub const SCALE: usize = 15;

#[derive(Clone)]
pub struct Monitor {
    cols: u8,
    rows: u8,
    pub buffer: [u8; 2048],
}

impl Monitor {
    // Functional methods
    pub fn new_default() -> Self {
        Self {
            cols: COLS as u8,
            rows: ROWS as u8,
            buffer: [0; COLS * ROWS],
        }
    }
    #[inline]
    pub fn toggle_pixel(&mut self, mut x: usize, mut y: usize) -> bool {
        x %= 64;
        y %= 32;
        self.buffer[x + (y * self.cols as usize)] ^= 1;
        self.buffer[x + (y * self.cols as usize)] == 0
    }

    pub fn clear(&mut self) {
        self.buffer = [0; COLS * ROWS]
    }

    // Utility methods
    #[inline]
    pub fn get_scaled_res(&self) -> (u32, u32) {
        (
            ((self.cols as usize) * SCALE) as u32,
            ((self.rows as usize) * SCALE) as u32,
        )
    }

    #[inline]
    pub fn get_buffer(&self) -> [u8; 2048] {
        self.buffer
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_pixels() {
        let mut monitor = Monitor::new_default();
        let mut i = 0;
        let mut j = 0;
        for _ in monitor.buffer.clone().iter() {
            monitor.toggle_pixel(j, i);
            j += 1;
            if j % 64 == 0 {
                j = 0;
                i += 1;
            }
        }
        let arr = [1; COLS * ROWS];
        assert_eq!(monitor.buffer, arr);
        monitor.clear();
        assert_eq!(monitor.buffer, [0; 2048]);
        for _ in monitor.buffer.clone().iter() {
            monitor.toggle_pixel(j, i);
            j += 1;
            if j % 64 == 0 {
                j = 0;
                i += 1;
            }
        }
        let arr = [1; COLS * ROWS];
        assert_eq!(monitor.buffer, arr);
    }

    #[test]
    fn out_of_bounds() {
        let mut monitor = Monitor::new_default();
        monitor.toggle_pixel(128, 64);
        assert_eq!(monitor.buffer[0], 1);
        monitor.toggle_pixel(128, 64);
        assert_eq!(monitor.buffer[0], 0);
        monitor.toggle_pixel(127, 64);
        assert_eq!(monitor.buffer[63], 1);
        monitor.toggle_pixel(127, 63);
        assert_eq!(monitor.buffer[63 + 31 * COLS], 1);
    }
}
