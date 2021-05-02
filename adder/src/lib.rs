pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn it_adds_two_2() {
        assert_eq!(add_two(10), 12)
    }

    #[test]
    fn it_adds_two_3() {
        assert_ne!(add_two(2), 5);
    }

    #[test]
    #[should_panic]
    fn it_adds_two_number_too_big() {
        add_two(i32::MAX);
    }

    #[test]
    #[should_panic]
    fn another_test() {
        panic!("Make this test fail");
    }
}
