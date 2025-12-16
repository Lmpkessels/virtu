use virtu::vm::virtual_machine::{ 
    Op, VirtualMachine 
};

use virtu::stack::Stack;

#[test]
fn test_opperation_add_in_virtual_machine_loop() {
    let mut vm = VirtualMachine::new();

    let program = [
        Op::Push(23),
        Op::Push(1444),
        Op::Add,
        Op::Push(1510),
        Op::Add,
        Op::Halt
    ];

    vm.run(&program);

    let result = vm.stack.pop();
    let expected = 23 + 1444 + 1510;

    assert_eq!((expected), (result));
}