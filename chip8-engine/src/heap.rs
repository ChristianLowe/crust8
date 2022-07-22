
const MEMORY_SIZE: usize = 4096;
const SIGILS_LENGTH: usize = 80;

pub const OFFSET_FONT: usize = 0x050;
pub const OFFSET_DATA: usize = 0x200;

const FONT_SIGILS: [u8; SIGILS_LENGTH] = [
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

pub struct Heap {
    elements: [u8; MEMORY_SIZE]
}

impl Heap {
    pub fn new(program_bytes: Vec<u8>) -> Self {
        let mut elements: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

        for i in 0..SIGILS_LENGTH {
            elements[OFFSET_FONT + i] = FONT_SIGILS[i];
        }

        for i in 0..program_bytes.len() {
            elements[OFFSET_DATA + i] = program_bytes[i];
        }

        Heap { elements }
    }

    pub fn set_byte(&mut self, index: usize, value: u8) {
        assert!(index < MEMORY_SIZE, "Attempt to set byte outside of memory");
        self.elements[index] = value;
    }

    pub fn set_bytes(&mut self, index: usize, values: &[u8]) {
        for i in 0..values.len() {
            self.set_byte(index + i, values[i]);
        }
    }

    pub fn set_as_decimal(&mut self, index: usize, value: u8) {
        self.set_byte(index + 0, value / 100);
        self.set_byte(index + 1, (value / 10) % 10);
        self.set_byte(index + 2, (value % 100) % 10);
    }

    pub fn get_bytes(&self, index: usize, len: usize) -> &[u8] {
        &self.elements[index..=(index + len)]
    }

    pub fn get_all_bytes(&self) -> &[u8] {
        &self.elements[..]
    }

    pub fn get_sprite(&self, index: usize, sprite_height: u8) -> &[u8] {
        let end = index + sprite_height as usize;
        &self.elements[index..end]
    }
}
