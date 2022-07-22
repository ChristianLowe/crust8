
const MAX_ELEMENTS: usize = 16;

pub struct Stack {
    elements: [usize; MAX_ELEMENTS],
    pointer: usize
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            elements: [0; MAX_ELEMENTS],
            pointer: 0
        }
    }

    pub fn push(&mut self, program_counter: usize) {
        assert!(self.pointer < MAX_ELEMENTS, "Max stack size reached");
        self.elements[self.pointer] = program_counter;
        self.pointer += 1;
    }

    pub fn pop(&mut self) -> usize {
        assert!(self.pointer > 0, "Attempt to pop from empty stack");
        self.pointer -= 1;
        self.elements[self.pointer]
    }
}
