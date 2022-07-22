const PIXELS_H: usize = 64;
const PIXELS_V: usize = 32;
const BUFFER_SIZE: usize = PIXELS_H * PIXELS_V;

const DARK_COLOR: (u8, u8, u8) = (0, 33, 66);
const LIGHT_COLOR: (u8, u8, u8) = (0, 128, 255);

pub struct Display {
    bits: [bool; BUFFER_SIZE]
}

impl Display {
    pub fn new() -> Self {
        Display {
            bits: [false; BUFFER_SIZE]
        }
    }

    pub fn clear(&mut self) {
        self.bits.fill(false);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: bool) -> bool {
        if x < PIXELS_H && y < PIXELS_V {
            let index = (PIXELS_H * y) + x;
            let is_collision = self.bits[index] && value;
            self.bits[index] ^= value;
            return is_collision;
        }
        false
    }

    pub fn render_sprite(&mut self, start_x: usize, start_y: usize, sprite: &[u8]) -> bool {
        let mut is_collision = false;
        for y in 0..sprite.len() {
            let row = sprite[y];
            for x in 0..8usize {
                let inverse = 7 - x;
                let is_lit = (row & (1u8 << inverse)) != 0;
                is_collision |= self.set_pixel(start_x + x, start_y + y, is_lit);
            }
        }
        is_collision
    }

    pub fn draw(&self, buffer: &mut [u8]) {
        for i in 0..BUFFER_SIZE {
            let is_pixel_set = self.bits[i];

            let i = i * 4;
            buffer[i + 0] = if is_pixel_set {LIGHT_COLOR.0} else {DARK_COLOR.0};
            buffer[i + 1] = if is_pixel_set {LIGHT_COLOR.1} else {DARK_COLOR.1};
            buffer[i + 2] = if is_pixel_set {LIGHT_COLOR.2} else {DARK_COLOR.2};
            buffer[i + 3] = 255;
        }
    }
}
