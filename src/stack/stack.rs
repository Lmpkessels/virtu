/// Stack memory structure, which uses push and pop to push values on top of 
/// the stack and pops values of the stack when needed.
///
/// Works as any stack memory structure with LIFO (Last in First out).
const STACK_SIZE: usize = 1024;

#[derive(Debug)]
pub struct Stack {
    pub data_storage: [u64; STACK_SIZE],
    pub sp: usize,
}

impl Stack {
    pub fn new() -> Self { Stack {
        data_storage: [0; STACK_SIZE],
        sp: 0,
    }}

    pub fn push(&mut self, value: u64) {
        self.data_storage[self.sp] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> u64 {
        self.sp -= 1;
        self.data_storage[self.sp]
    }
}