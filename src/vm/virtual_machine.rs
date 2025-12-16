use crate::stack::Stack;
use crate::alu::{ AluOp, Alu };
use crate::pc::ProgramCounter;

pub struct VirtualMachine {
    pub stack: Stack,
    pub alu: Alu,
    pub pc: ProgramCounter,
    pub running: bool
}

pub enum Op {
    Push(u64),
    Add,
    Sub,
    Mul,
    Div,
    Halt
}

/// Virtual machine execution loop, works on an array with the program.
/// Within the array are the 'enum Op' opperation, which make use of the:
/// Stack, Program Counter (PC), Arithmetic Logic Unit (ALU). 
impl VirtualMachine {
    pub fn new() -> Self { Self {
        stack: Stack::new(),
        alu: Alu::new(),
        pc: ProgramCounter::new(),
        running: true
    }}

    pub fn run(&mut self, program: &[Op]) {
        self.pc.reset();
        self.running = true;

        while self.running {
            let instr = &program[self.pc.get()];
            self.pc.inc();

            match instr {
                Op::Push(v) => {
                    self.stack.push(*v);
                }

                Op::Add => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    let op = self.alu.alu(a, b, AluOp::Add);
                    self.stack.push(op);
                }

                Op::Sub => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    let op = self.alu.alu(a, b, AluOp::Sub);
                    self.stack.push(op);
                }

                Op::Mul => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    let op = self.alu.alu(a, b, AluOp::Mul);
                    self.stack.push(op);
                }
                
                Op::Div => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    let op = self.alu.alu(a, b, AluOp::Div);
                    self.stack.push(op);
                }

                Op::Halt => {
                    self.running = false;
                }
            }
        }
    }
}

