const A: [u32; 8] = [
    0x12, 0x41, 0x55, 0x22,
    0xAB, 0xCD, 0xEF, 0x99,
];

const M: [u32; 8] = [
    0x1312, 0xAFF1, 0xF11A, 0x1121,
    0x3312, 0xCFF1, 0xBD1A, 0xA121,
];


use virtu::alu::{
    alu, AluOp
};

#[test]
fn test_arithmetic_ops() {
    for &x in &A {
        for &y in &A {
            if y != 0 {
                assert_eq!(alu(x, y, AluOp::Div), x / y);
            }

            assert_eq!(alu(x, y, AluOp::Add), x + y);

            if x >= y {
                assert_eq!(alu(x, y, AluOp::Sub), x - y);
            }
        }
    }

    for &x in &M {
        for &y in &M {
            assert_eq!(alu(x, y, AluOp::Mul), x * y);
        }
    }
}


