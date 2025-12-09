use virtu::stack::Stack;

#[test]
fn test_pop_data_of_stack() {
    let push_data = [
        0, 1341, 11134, 139498164, 11, 1341414
    ];

    let mut stack = Stack::new();

    for i in 0..push_data.len() {
        stack.push(push_data[i]);
    }
    
    let pop_stack_result0 = stack.pop();
    let pop_stack_result1 = stack.pop();
    let pop_stack_result2 = stack.pop();
    let pop_stack_result3 = stack.pop();
    let pop_stack_result4 = stack.pop();
    let pop_stack_result5 = stack.pop();

    let pop_stack_expected0 = 1341414;
    let pop_stack_expected1 = 11;
    let pop_stack_expected2 = 139498164;
    let pop_stack_expected3 = 11134;
    let pop_stack_expected4 = 1341;
    let pop_stack_expected5 = 0;

    assert_eq!((pop_stack_result0), (pop_stack_expected0));
    assert_eq!((pop_stack_result1), (pop_stack_expected1));
    assert_eq!((pop_stack_result2), (pop_stack_expected2));
    assert_eq!((pop_stack_result3), (pop_stack_expected3));
    assert_eq!((pop_stack_result4), (pop_stack_expected4));
    assert_eq!((pop_stack_result5), (pop_stack_expected5));
}