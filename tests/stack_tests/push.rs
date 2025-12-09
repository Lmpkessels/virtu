use virtu::stack::Stack;

#[test]
fn test_push_data_to_stack() {
    let push_data = [
        0, 1341, 11134, 139498164, 11, 1341414
    ];

    let mut result_stack = Stack::new();
    let mut expected_stack = [0u64; 1024];

    for i in 0..push_data.len() {
        result_stack.push(push_data[i]);
        expected_stack[i] = push_data[i];
    }

    assert_eq!((result_stack.data_storage), (expected_stack));
}