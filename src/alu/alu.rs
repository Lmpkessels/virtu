/// A simple ALU (Arithmetic Logic Unit) with a fixed set of rules.
use crate::alu::{
    add, sub, mul, div, and, or, xor, nand, nor, xnor
};

pub enum AluOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Nand,
    Nor,
    Xnor,
}

/// Works on an Chosen ALU Operation (Arithmetic Math, or Binary Logic) with
/// the parameters X and Y which are unsigned integers with a maximum word 
/// width of 32-bits (equals integer 4,294,967,295, 2^32 - 1).
pub fn alu(x: u32, y: u32, alu_op: AluOp) -> u32 {
    match alu_op {
        AluOp::Add => add(x, y),
        AluOp::Sub => sub(x, y),
        AluOp::Mul => mul(x, y),
        AluOp::Div => div(x, y),
        AluOp::And => and(x, y),
        AluOp::Or => or(x, y),
        AluOp::Xor => xor(x, y),
        AluOp::Nand => nand(x, y),
        AluOp::Nor => nor(x, y),
        AluOp::Xnor => xnor(x, y),
    }
}