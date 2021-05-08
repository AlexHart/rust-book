//! Library to add 2 to a value.

/// Adds two to the number given.
/// 
/// # Examples
///
/// ```
/// let arg = 2;
/// let answer = add_two::add_two(arg);
///
/// assert_eq!(4, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_two(4), 6);
    }
}
