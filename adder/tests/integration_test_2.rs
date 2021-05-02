use adder;

#[test]
#[should_panic]
fn it_adds_two_to_max_i32_panics() {
    adder::add_two(i32::MAX);
}

#[test]
fn it_adds_two_to_number_below_zero() {
    assert_eq!(-2, adder::add_two(-4));
}

#[test]
fn it_adds_two_to_zero() {
    assert_eq!(2, adder::add_two(0));
}
