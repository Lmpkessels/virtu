/// Bit-by-bit binary ops for ALU.
/// Each functions works bit-by-bit on the given arguments.
pub fn and(x: u32, y: u32) -> u32 {
    x & y
}

pub fn or(x: u32, y: u32) -> u32 {
    x | y
}

pub fn xor(x: u32, y: u32) -> u32 {
    x ^ y
}

pub fn not(x: u32) -> u32 {
    !x
}

pub fn nand(x: u32, y: u32) -> u32 {
    not(and(x, y))
}

pub fn nor(x: u32, y: u32) -> u32 {
    not(or(x, y))
}

pub fn xnor(x: u32, y: u32) -> u32 {
    not(xor(x, y))
}