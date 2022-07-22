use crate::registers::Register;

#[derive(Copy, Clone)]
pub struct Word(pub u16);

impl Word {
    /// In Chip-8, words consist of two bytes in Big-Endian order.
    /// This method loads in a word from the memory at the given address, and panics on failure.
    pub fn new(memory: &[u8], pc: usize) -> Self {
        assert!(pc + 1 < memory.len(), "Expecting two free bytes at pc location");
        Word(((memory[pc] as u16) << 8) | (memory[pc + 1] as u16))
    }

    /// Returns the first nibble of the word, used as a control bit to determine the opcode.
    /// "C" is the 4 bits represented by the exclamation mark in: 0x!???
    pub fn c(self) -> u8 {
        ((self.0 >> 12) & 0xF) as u8
    }

    /// Returns the second nibble of the word, used as a register # in some opcodes.
    /// "X" is the 4 bits represented by the exclamation mark in: 0x?!??
    pub fn x(self) -> Register {
        Register::new(((self.0 >> 8) & 0xF) as u8)
    }

    /// Returns the third nibble of the word, used as a register # in some opcodes.
    /// "Y" is the 4 bits represented by the exclamation mark in: 0x??!?
    pub fn y(self) -> Register {
        Register::new(((self.0 >> 4) & 0xF) as u8)
    }

    /// Returns the fourth nibble of the word, used as a constant value in some opcodes.
    /// "N" is the 4 bits represented by the exclamation mark in: 0x???!
    pub fn n(self) -> u8 {
        (self.0 & 0xF) as u8
    }

    /// Returns the back two nibbles of the word, used as a constant value in some opcodes.
    /// "NN" is the 8 bits represented by the exclamation marks in: 0x??!!
    pub fn nn(self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Returns the back three nibbles of the word, used as a constant address in some opcodes.
    /// "NNN" is the 12 bits represented by the exclamation marks in: 0x?!!!
    pub fn nnn(self) -> u16 {
        self.0 & 0xFFF
    }
}
