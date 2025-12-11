use virtu::pc::ProgramCounter;

#[test]
fn test_program_counter_operations() {
    let mut pc = ProgramCounter::new();

    assert_eq!((pc.value()), 0);

    pc.load(12111);
    assert_eq!((pc.value()), 12111);

    pc.reset();
    assert_eq!((pc.value()), 0);

    let values = [
        1212, 8314, 11873, 189431, 111, 13814
    ];

    for &value in &values {
        pc.load(value);
        assert_eq!((pc.value()), (value));
    }

    pc.reset();
    assert_eq!((pc.value()), 0);
}

#[test]
fn test_increment_chain() {
    let mut pc = ProgramCounter::new();
    for i in 0..1000000 {
        pc.increment();
        assert_eq!((pc.value()), (i));
    }
}