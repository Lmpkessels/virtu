/// Bit-by-bit binary ops for ALU.
/// Each functions works bit-by-bit on the given arguments.
fn and(x: u32, y: u32) -> u32 {
    x & y
}

fn or(x: u32, y: u32) -> u32 {
    x | y
}

fn xor(x: u32, y: u32) -> u32 {
    x ^ y
}

fn not(x: u32) -> u32 {
    !x
}

fn nand(x: u32, y: u32) -> u32 {
    not(and(x, y))
}

fn nor(x: u32, y: u32) -> u32 {
    not(or(x, y))
}

fn xnor(x: u32, y: u32) -> u32 {
    not(xor(x, y))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all_binary_functions_to_check_if_all_opperations_match() {
        let values = [
            0x00000000,
            0xFFFFFFFF,
            0xAAAAAAAA,
            0x55555555,
            0x12345678,
            0x87654321,
            0b0001_0000,
            0b1110_1110,
        ];

        for &x in &values {
            for &y in &values {
                let exp_and = x & y;
                let exp_or = x | y;
                let exp_xor = x ^ y;
                let exp_not = !x;
                let exp_nand = !(x & y);
                let exp_nor = !(x | y);
                let exp_xnor = !(x ^ y);

                assert_eq!((and(x, y)), (exp_and));
                assert_eq!((or(x, y)), (exp_or));
                assert_eq!((xor(x, y)), (exp_xor));
                assert_eq!((not(x)), (exp_not));
                assert_eq!((nand(x, y)), (exp_nand));
                assert_eq!((nor(x, y)), (exp_nor));
                assert_eq!((xnor(x, y)), (exp_xnor));
            }
        }
    }
}