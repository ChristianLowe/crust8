use crate::instruction::Instruction::*;
use crate::registers::Register;
use crate::word::Word;

/// Represents an instruction loaded from the Chip8 program.
/// Documentation credit: https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    /// This instruction is invalid or unsupported by the emulator
    Unimplemented { opcode: u16 },

    /// Indicates an end to the program execution
    EndProgram,

    /// Clear the screen
    ClearScreen,

    /// Return from a subroutine
    ReturnSubroutine,

    /// Jump to address `NNN`
    Goto { address: u16 },

    /// Execute subroutine starting at address `NNN`
    CallSubroutine { address: u16 },

    /// Skip the following instruction if the value of register `VX` equals `NN`
    SkipIfValueEq { register: Register, value: u8 },

    /// Skip the following instruction if the value of register `VX` is not equal to `NN`
    SkipIfValueNe { register: Register, value: u8 },

    /// Skip the following instruction if the value of register `VX` is equal to the value of register `VY`
    SkipIfRegistersEq { register_x: Register, register_y: Register },

    /// Store number `NN` in register `VX`
    RegisterValueStore { register: Register, value: u8 },

    /// Add the value `NN` to register `VX`
    RegisterValueAdd { register: Register, value: u8 },

    /// Store the value of register `VY` in register `VX`
    RegistersCopy { register_to: Register, register_from: Register },

    /// Set `VX` to `VX` OR `VY`
    RegistersOrEq { register_to: Register, register_from: Register },

    /// Set `VX` to `VX` AND `VY`
    RegistersAndEq { register_to: Register, register_from: Register },

    /// Set `VX` to `VX` XOR `VY`
    RegistersXorEq { register_to: Register, register_from: Register },

    /// Add the value of register `VY` to register `VX`
    /// Set `VF` to 01 if a carry occurs
    /// Set `VF` to 00 if a carry does not occur
    RegistersAdd { register_to: Register, register_from: Register },

    /// Subtract the value of register `VY` from register `VX`
    /// Set `VF` to 00 if a borrow occurs
    /// Set `VF` to 01 if a borrow does not occur
    RegistersSub { register_to: Register, register_from: Register },

    /// Store the value of register `VY` shifted right one bit in register `VX`
    /// Set register `VF` to the least significant bit prior to the shift
    /// `VY` is unchanged
    RegistersShiftRightEq { register_to: Register, register_from: Register },

    /// Set register `VX` to the value of `VY` minus `VX`
    /// Set `VF` to 00 if a borrow occurs
    /// Set `VF` to 01 if a borrow does not occur
    RegistersSubReversed { register_to: Register, register_from: Register },

    /// Store the value of register `VY` shifted left one bit in register `VX`
    /// Set register `VF` to the most significant bit prior to the shift
    /// `VY` is unchanged
    RegistersShiftLeftEq { register_to: Register, register_from: Register },

    /// Skip the following instruction if the value of register `VX` is not equal to the value of register `VY`
    SkipIfRegistersNe { register_x: Register, register_y: Register },

    /// Store memory address `NNN` in register `I`
    IStoreAddress { address: u16 },

    /// Jump to address `NNN + V0`
    GotoOffsetted { address: u16 },

    /// Set `VX` to a random number with a mask of `NN`
    RegisterStoreRandom { register: Register, mask: u8 },

    /// Draw a sprite at position `VX`, `VY` with `N` bytes of sprite data starting at the address stored in `I`
    /// Set `VF` to 01 if any set pixels are changed to unset, and 00 otherwise
    DrawSprite { register_x: Register, register_y: Register, sprite_height: u8 },

    /// Skip the following instruction if the key corresponding to the hex value currently stored in register `VX` is pressed
    SkipIfKeyOn { register: Register },

    /// Skip the following instruction if the key corresponding to the hex value currently stored in register `VX` is not pressed
    SkipIfKeyOff { register: Register },

    /// Store the current value of the delay timer in register `VX`
    DelayTimerToRegister { register: Register },

    /// Wait for a keypress and store the result in register `VX`
    WaitForAnyKey { register: Register },

    /// Set the delay timer to the value of register `VX`
    RegisterToDelayTimer { register: Register },

    /// Set the sound timer to the value of register `VX`
    RegisterToSoundTimer { register: Register },

    /// Add the value stored in register `VX` to register `I`
    IAddOffset { register: Register },

    /// Set `I` to the memory address of the sprite data corresponding to the hexadecimal digit stored in register `VX`
    IStoreDigitAddress { register: Register },

    /// Store the binary-coded decimal equivalent of the value stored in register `VX` at addresses `I`, `I + 1`, and `I + 2`
    /// See also: https://en.wikipedia.org/wiki/Binary-coded_decimal
    HexToDecimal { register: Register },

    /// Store the values of registers `V0` to `VX` inclusive in memory starting at address `I`
    /// `I` is set to `I + X + 1` after operation
    RegistersDump { max_register: Register },

    /// Fill registers `V0` to `VX` inclusive with the values stored in memory starting at address `I`
    /// `I` is set to `I + X + 1` after operation
    RegistersLoad { max_register: Register },
}

impl Instruction {
    pub fn new(memory: &[u8], pc: usize) -> Self {
        assert!(pc + 1 < memory.len(), "Expecting two free bytes at pc location");

        let word = Word::new(memory, pc);
        match word.c() {
            0x0 => match word.nnn() {
                0x000 | 0x0DE => EndProgram,
                0x0E0 => ClearScreen,
                0x0EE => ReturnSubroutine,
                _ => Unimplemented { opcode: word.0 },
            },
            0x1 => Goto { address: word.nnn() },
            0x2 => CallSubroutine { address: word.nnn() },
            0x3 => SkipIfValueEq { register: word.x(), value: word.nn() },
            0x4 => SkipIfValueNe { register: word.x(), value: word.nn() },
            0x5 => SkipIfRegistersEq { register_x: word.x(), register_y: word.y() },
            0x6 => RegisterValueStore { register: word.x(), value: word.nn() },
            0x7 => RegisterValueAdd { register: word.x(), value: word.nn() },
            0x8 => {
                let register_to: Register = word.x();
                let register_from: Register = word.y();
                match word.n() {
                    0x0 => RegistersCopy { register_to, register_from },
                    0x1 => RegistersOrEq { register_to, register_from },
                    0x2 => RegistersAndEq { register_to, register_from },
                    0x3 => RegistersXorEq { register_to, register_from },
                    0x4 => RegistersAdd { register_to, register_from },
                    0x5 => RegistersSub { register_to, register_from },
                    0x6 => RegistersShiftRightEq { register_to, register_from },
                    0x7 => RegistersSubReversed { register_to, register_from },
                    0xE => RegistersShiftLeftEq { register_to, register_from },
                    _ => Unimplemented { opcode: word.0 },
                }
            }
            0x9 => SkipIfRegistersNe { register_x: word.x(), register_y: word.y() },
            0xA => IStoreAddress { address: word.nnn() },
            0xB => GotoOffsetted { address: word.nnn() },
            0xC => RegisterStoreRandom { register: word.x(), mask: word.nn() },
            0xD => DrawSprite { register_x: word.x(), register_y: word.y(), sprite_height: word.n() },
            0xE => match word.nn() {
                0x9E => SkipIfKeyOn { register: word.x() },
                0xA1 => SkipIfKeyOff { register: word.x() },
                _ => Unimplemented { opcode: word.0 },
            },
            0xF => {
                let register = word.x();
                match word.nn() {
                    0x07 => DelayTimerToRegister { register },
                    0x0A => WaitForAnyKey { register },
                    0x15 => RegisterToDelayTimer { register },
                    0x18 => RegisterToSoundTimer { register },
                    0x1E => IAddOffset { register },
                    0x29 => IStoreDigitAddress { register },
                    0x33 => HexToDecimal { register },
                    0x55 => RegistersDump { max_register: register },
                    0x65 => RegistersLoad { max_register: register },
                    _ => Unimplemented { opcode: word.0 },
                }
            },

            _ => panic!("Unreachable code")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_instr(opcode: u16) -> Instruction {
        let mem =
        [
            (opcode >> 8) as u8,
            opcode as u8,
        ];
        Instruction::new(&mem, 0)
    }

    #[test]
    pub fn test_c0() {
        let instr = get_instr(0x00E0);
        assert_eq!(instr, ClearScreen);

        let instr = get_instr(0x00EE);
        assert_eq!(instr, ReturnSubroutine);
    }

    #[test]
    pub fn test_c1() {
        let instr = get_instr(0x1234);
        assert_eq!(instr, Goto {address: 0x234});
    }

    #[test]
    pub fn test_c2() {
        let instr = get_instr(0x2345);
        assert_eq!(instr, CallSubroutine {address: 0x345});
    }

    #[test]
    pub fn test_c3() {
        let instr = get_instr(0x3A11);
        assert_eq!(instr, SkipIfValueEq {
            register: Register::new(0xA),
            value: 0x11
        });
    }

    #[test]
    pub fn test_c4() {
        let instr = get_instr(0x4A11);
        assert_eq!(instr, SkipIfValueNe {
            register: Register::new(0xA),
            value: 0x11
        });
    }

    #[test]
    pub fn test_c5() {
        let instr = get_instr(0x5AB0);
        assert_eq!(instr, SkipIfRegistersEq {
            register_x: Register::new(0xA),
            register_y: Register::new(0xB),
        });
    }

    #[test]
    pub fn test_c6() {
        let instr = get_instr(0x6FAB);
        assert_eq!(instr, RegisterValueStore {
            register: Register::new(0xF),
            value: 0xAB,
        });
    }

    #[test]
    pub fn test_c7() {
        let instr = get_instr(0x7042);
        assert_eq!(instr, RegisterValueAdd {
            register: Register::first(),
            value: 0x42,
        });
    }

    #[test]
    pub fn test_c8() {
        let instr = get_instr(0x8AF0);
        assert_eq!(instr, RegistersCopy {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });

        let instr = get_instr(0x8AF1);
        assert_eq!(instr, RegistersOrEq {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });

        let instr = get_instr(0x8AF2);
        assert_eq!(instr, RegistersAndEq {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });

        let instr = get_instr(0x8AF3);
        assert_eq!(instr, RegistersXorEq {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });

        let instr = get_instr(0x8AF4);
        assert_eq!(instr, RegistersAdd {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });

        let instr = get_instr(0x8AF5);
        assert_eq!(instr, RegistersSub {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });

        let instr = get_instr(0x8AF6);
        assert_eq!(instr, RegistersShiftRightEq {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });

        let instr = get_instr(0x8AF7);
        assert_eq!(instr, RegistersSubReversed {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });

        let instr = get_instr(0x8AFE);
        assert_eq!(instr, RegistersShiftLeftEq {
            register_to: Register::new(0xA),
            register_from: Register::new(0xF),
        });
    }

    #[test]
    pub fn test_c9() {
        let instr = get_instr(0x90F0);
        assert_eq!(instr, SkipIfRegistersNe {
            register_x: Register::first(),
            register_y: Register::new(0xF),
        });
    }

    #[test]
    pub fn test_c10() {
        let instr = get_instr(0xA09F);
        assert_eq!(instr, IStoreAddress {address: 0x09F});
    }

    #[test]
    pub fn test_c11() {
        let instr = get_instr(0xBABE);
        assert_eq!(instr, GotoOffsetted {address: 0xABE});
    }

    #[test]
    pub fn test_c12() {
        let instr = get_instr(0xCABE);
        assert_eq!(instr, RegisterStoreRandom {
            register: Register::new(0xA),
            mask: 0xBE,
        });
    }

    #[test]
    pub fn test_c13() {
        let instr = get_instr(0xD123);
        assert_eq!(instr, DrawSprite {
            register_x: Register::new(0x1),
            register_y: Register::new(0x2),
            sprite_height: 0x3,
        });
    }

    #[test]
    pub fn test_c14() {
        let instr = get_instr(0xE09E);
        assert_eq!(instr, SkipIfKeyOn {register: Register::first()});

        let instr = get_instr(0xE0A1);
        assert_eq!(instr, SkipIfKeyOff {register: Register::first()});
    }

    #[test]
    pub fn test_c15() {
        let instr = get_instr(0xF007);
        assert_eq!(instr, DelayTimerToRegister {register: Register::first()});

        let instr = get_instr(0xF00A);
        assert_eq!(instr, WaitForAnyKey {register: Register::first()});

        let instr = get_instr(0xF015);
        assert_eq!(instr, RegisterToDelayTimer {register: Register::first()});

        let instr = get_instr(0xF018);
        assert_eq!(instr, RegisterToSoundTimer {register: Register::first()});

        let instr = get_instr(0xF01E);
        assert_eq!(instr, IAddOffset {register: Register::first()});

        let instr = get_instr(0xF029);
        assert_eq!(instr, IStoreDigitAddress {register: Register::first()});

        let instr = get_instr(0xF033);
        assert_eq!(instr, HexToDecimal {register: Register::first()});

        let instr = get_instr(0xF055);
        assert_eq!(instr, RegistersDump {max_register: Register::first()});

        let instr = get_instr(0xF065);
        assert_eq!(instr, RegistersLoad {max_register: Register::first()});
    }
}
