pub struct ProgramCounter {
    pub value: usize
}

/// A simple program counter with its functions, to load in Data, Increment data
/// by +1, check what value is hold within register, and reset to reset register.
impl ProgramCounter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn load(&mut self, addr: usize) {
        self.value = addr;
    }

    pub fn inc(&mut self) {
        self.value += 1;
    }
    
    pub fn get(&self) -> usize {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}