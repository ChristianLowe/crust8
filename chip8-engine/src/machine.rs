use log::*;
use crate::display::Display;
use crate::Quirks;
use crate::{heap, heap::Heap};
use crate::instruction::Instruction;
use crate::registers::{Register, Registers};
use crate::stack::Stack;
use crate::timers::Timers;

pub struct Machine {
    heap: Heap,
    stack: Stack,
    registers: Registers,
    timers: Timers,
    display: Display,
    quirks: Quirks,
}

impl Machine {
    pub fn new(program_bytes: Vec<u8>, quirks: Quirks) -> Self {
        Machine {
            heap: Heap::new(program_bytes),
            stack: Stack::new(),
            registers: Registers::new(),
            timers: Timers::new(),
            display: Display::new(),
            quirks
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        self.display.draw(frame);
    }

    pub fn tick(&mut self, keys_pressed: Vec<u8>) {
        let mut pc = self.registers.program_counter;
        let mut pause = false;

        let instruction = Instruction::new(self.heap.get_all_bytes(), pc);
        match instruction {
            Instruction::Unimplemented {opcode} =>
                warn!("Unimplemented instruction detected: {:#06x}", opcode),
            Instruction::EndProgram =>
                pause = true,
            Instruction::ClearScreen =>
                self.display.clear(),
            Instruction::ReturnSubroutine =>
                pc = self.stack.pop() + 2,
            Instruction::Goto { address } => {
                if pc == address as usize {
                    pause = true;
                } else {
                    pc = address as usize;
                }
            }
            Instruction::CallSubroutine { address } => {
                self.stack.push(pc);
                pc = address as usize;
            }
            Instruction::SkipIfValueEq { register, value } =>
                if self.registers.get_value(register) == value {
                    pc += 4;
                },
            Instruction::SkipIfValueNe { register, value } =>
                if self.registers.get_value(register) != value {
                    pc += 4;
                },
            Instruction::SkipIfRegistersEq { register_x, register_y } =>
                if self.registers.get_value(register_x) == self.registers.get_value(register_y) {
                    pc += 4;
                },
            Instruction::RegisterValueStore { register, value } =>
                self.registers.set_value(register, value),
            Instruction::RegisterValueAdd { register, value } =>
                self.registers.add_value(register, value),
            Instruction::RegistersCopy { register_to, register_from } =>
                self.registers.copy_registers(register_to, register_from),
            Instruction::RegistersOrEq { register_to, register_from } =>
                self.registers.or_registers(register_to, register_from),
            Instruction::RegistersAndEq { register_to, register_from } =>
                self.registers.and_registers(register_to, register_from),
            Instruction::RegistersXorEq { register_to, register_from } =>
                self.registers.xor_registers(register_to, register_from),
            Instruction::RegistersAdd { register_to, register_from } =>
                self.registers.add_registers(register_to, register_from),
            Instruction::RegistersSub { register_to, register_from } =>
                self.registers.sub_registers(register_to, register_from),
            Instruction::RegistersShiftRightEq { register_to, register_from } =>
                self.registers.shr_registers(register_to, register_from, self.quirks.is_lazy_shift),
            Instruction::RegistersSubReversed { register_to, register_from } =>
                self.registers.sub_registers_reversed(register_to, register_from),
            Instruction::RegistersShiftLeftEq { register_to, register_from } =>
                self.registers.shl_registers(register_to, register_from, self.quirks.is_lazy_shift),
            Instruction::SkipIfRegistersNe { register_x, register_y } =>
                if self.registers.get_value(register_x) != self.registers.get_value(register_y) {
                    pc += 4;
                },
            Instruction::IStoreAddress { address } =>
                self.registers.index = address as usize,
            Instruction::GotoOffsetted { address } => {
                let offset = self.registers.get_value(Register::first()) as usize;
                let adjusted_address = address as usize + offset;
                if pc == adjusted_address {
                    pause = true;
                } else {
                    pc = adjusted_address;
                }
            }
            Instruction::RegisterStoreRandom { register, mask } =>
                self.registers.set_value(register, fastrand::u8(..) & mask),
            Instruction::DrawSprite { register_x, register_y, sprite_height } => {
                let sprite = self.heap.get_sprite(self.registers.index, sprite_height);
                let x = self.registers.get_value(register_x) as usize;
                let y = self.registers.get_value(register_y) as usize;
                let is_collision = self.display.render_sprite(x, y, sprite);
                self.registers.set_flag(is_collision);
            }
            Instruction::SkipIfKeyOn { register } =>
                if keys_pressed.contains(&self.registers.get_value(register)) {
                    pc += 4;
                }
            Instruction::SkipIfKeyOff { register } =>
                if !keys_pressed.contains(&self.registers.get_value(register)) {
                    pc += 4;
                }
            Instruction::DelayTimerToRegister { register } =>
                self.registers.set_value(register, self.timers.delay),
            Instruction::WaitForAnyKey { register } => {
                if keys_pressed.is_empty() {
                    pause = true;
                } else {
                    pause = false;
                    self.registers.set_value(register, keys_pressed[0]);
                }
            }
            Instruction::RegisterToDelayTimer { register } =>
                self.timers.delay = self.registers.get_value(register),
            Instruction::RegisterToSoundTimer { register } =>
                self.timers.sound = self.registers.get_value(register),
            Instruction::IAddOffset { register } =>
                self.registers.index += self.registers.get_value(register) as usize,
            Instruction::IStoreDigitAddress { register } => {
                let digit = self.registers.get_value(register) as usize;
                self.registers.index = heap::OFFSET_FONT + (digit * 5);
            }
            Instruction::HexToDecimal { register } =>
                self.heap.set_as_decimal(self.registers.index, self.registers.get_value(register)),
            Instruction::RegistersDump { max_register } => {
                self.heap.set_bytes(self.registers.index, self.registers.dump(max_register));
                if !self.quirks.is_static_dump_index {
                    self.registers.index += max_register.idx() + 1;
                }
            }
            Instruction::RegistersLoad { max_register } => {
                self.registers.load(self.heap.get_bytes(self.registers.index, max_register.idx()));
                if !self.quirks.is_static_dump_index {
                    self.registers.index += max_register.idx() + 1;
                }
            }
        }

        self.timers.tick();

        if !pause && pc == self.registers.program_counter {
            // By default, increment the program counter by two bytes (one word length).
            pc += 2;
        }
        self.registers.program_counter = pc;
    }
}
