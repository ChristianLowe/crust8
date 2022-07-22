use crate::heap;

const GENERAL_REGISTER_COUNT: usize = 16;
const FLAG_REGISTER_IDX: u8 = 0xF;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Register(usize);

impl Register {
    pub fn new(register: u8) -> Self {
        assert!((register as usize) < GENERAL_REGISTER_COUNT, "Attempt to access non-existent general register");
        Register(register as usize)
    }

    pub fn flag() -> Self {
        Register(FLAG_REGISTER_IDX as usize)
    }

    pub fn first() -> Self {
        Register(0)
    }

    pub fn idx(self) -> usize {
        self.0
    }
}

pub struct Registers {
    general: [u8; GENERAL_REGISTER_COUNT],
    pub index: usize,
    pub program_counter: usize,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            general: [0; GENERAL_REGISTER_COUNT],
            index: 0,
            program_counter: heap::OFFSET_DATA,
        }
    }

    pub fn get_value(&self, register: Register) -> u8 {
        self.general[register.idx()]
    }

    pub fn set_value(&mut self, register: Register, value: u8) {
        self.general[register.idx()] = value;
    }

    pub fn add_value(&mut self, register: Register, value: u8) {
        let current = self.get_value(register);
        let new = current.wrapping_add(value);
        self.set_value(register, new);
    }

    pub fn copy_registers(&mut self, to: Register, from: Register) {
        self.set_value(to, self.get_value(from));
    }

    pub fn or_registers(&mut self, to: Register, from: Register) {
        let to_val = self.get_value(to);
        let from_val = self.get_value(from);
        self.set_value(to, to_val | from_val);
    }

    pub fn and_registers(&mut self, to: Register, from: Register) {
        let to_val = self.get_value(to);
        let from_val = self.get_value(from);
        self.set_value(to, to_val & from_val);
    }

    pub fn xor_registers(&mut self, to: Register, from: Register) {
        let to_val = self.get_value(to);
        let from_val = self.get_value(from);
        self.set_value(to, to_val ^ from_val);
    }

    pub fn add_registers(&mut self, to: Register, from: Register) {
        let to_val = self.get_value(to);
        let from_val = self.get_value(from);
        self.set_flag((to_val as u16) + (from_val as u16) > 255);
        self.set_value(to, to_val.wrapping_add(from_val));
    }

    pub fn sub_registers(&mut self, to: Register, from: Register) {
        let to_val = self.get_value(to);
        let from_val = self.get_value(from);
        self.set_flag((to_val as i16) - (from_val as i16) >= 0);
        self.set_value(to, to_val.wrapping_sub(from_val));
    }

    pub fn sub_registers_reversed(&mut self, to: Register, from: Register) {
        let to_val = self.get_value(to);
        let from_val = self.get_value(from);
        self.set_flag((from_val as i16) - (to_val as i16) >= 0);
        self.set_value(to, from_val.wrapping_sub(to_val));
    }

    pub fn shr_registers(&mut self, to: Register, from: Register, is_lazy_shift: bool) {
        let from_val = if !is_lazy_shift {
            self.get_value(from) // VX = VY >> 1
        } else {
            self.get_value(to) // VX = VX >> 1
        };
        self.set_flag((from_val & 1) != 0);
        self.set_value(to, from_val >> 1);
    }

    pub fn shl_registers(&mut self, to: Register, from: Register, is_lazy_shift: bool) {
        let from_val = if !is_lazy_shift {
            self.get_value(from) // VX = VY << 1
        } else {
            self.get_value(to) // VX = VX << 1
        };
        self.set_flag((from_val & 0b1000_0000) != 0);
        self.set_value(to, from_val << 1);
    }

    pub fn set_flag(&mut self, enable: bool) {
        self.set_value(Register::flag(), enable as u8);
    }

    pub fn dump(&self, max_register: Register) -> &[u8] {
        &self.general[0..=max_register.idx()]
    }

    pub fn load(&mut self, bytes: &[u8]) {
        for i in 0..bytes.len() {
            self.general[i] = bytes[i];
        }
    }
}
