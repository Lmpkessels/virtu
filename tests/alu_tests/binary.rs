use virtu::alu::{
    and, or, xor, not, nand, nor, xnor, AluOp, alu
};

const B: [u32; 8] = [
    0x00000000,
    0xFFFFFFFF,
    0xAAAAAAAA,
    0x55555555,
    0x12345678,
    0x87654321,
    0b0001_0000,
    0b1110_1110,
];

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

#[test]
fn test_bitwise_ops() {
    for &x in &B {
        for &y in &B {
            assert_eq!(alu(x, y, AluOp::And), x & y);
            assert_eq!(alu(x, y, AluOp::Or),  x | y);
            assert_eq!(alu(x, y, AluOp::Xor), x ^ y);

            assert_eq!(alu(x, y, AluOp::Nand), !(x & y));
            assert_eq!(alu(x, y, AluOp::Nor),  !(x | y));
            assert_eq!(alu(x, y, AluOp::Xnor), !(x ^ y));
        }

        // Unary NOT
        assert_eq!(alu(x, 0, AluOp::Xor), x);
    }
}


