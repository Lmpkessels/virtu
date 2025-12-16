use virtu::alu::{ 
    AluOp, Alu 
};

#[test]
fn test_dispatch() {
    let mut alu = Alu::new();

    let x = 0x55AA;
    let y = 0x0F0F;

    let ops = [
        (AluOp::Add,  x + y),
        (AluOp::Sub,  x - y),
        (AluOp::Mul,  x * y),
        (AluOp::Div,  x / y),
        (AluOp::And,  x & y),
        (AluOp::Or,   x | y),
        (AluOp::Xor,  x ^ y),
        (AluOp::Nand, !(x & y)),
        (AluOp::Nor,  !(x | y)),
        (AluOp::Xnor, !(x ^ y)),
    ];

    for (op, expected) in ops {
        assert_eq!(alu.alu(x, y, op), expected);
    }
}

