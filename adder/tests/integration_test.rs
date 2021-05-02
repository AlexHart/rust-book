use adder;

#[test]
pub fn it_adds_two_from_test_folder() {
    assert_eq!(4, adder::add_two(2));
}
