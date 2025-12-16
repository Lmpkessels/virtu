/// Bit-by-bit binary ops for ALU.
/// Each functions works bit-by-bit on the given arguments.
pub fn and(x: u64, y: u64) -> u64 { x & y }

pub fn or(x: u64, y: u64) -> u64 { x | y }

pub fn xor(x: u64, y: u64) -> u64 { x ^ y }

pub fn not(x: u64) -> u64 { !x }

pub fn nand(x: u64, y: u64) -> u64 { not(and(x, y)) }

pub fn nor(x: u64, y: u64) -> u64 { not(or(x, y)) }

pub fn xnor(x: u64, y: u64) -> u64 { not(xor(x, y)) }