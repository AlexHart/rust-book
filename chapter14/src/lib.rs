//! # Chapter 14
//!
//! `chapter14` is a collection of utilities to make performing certain
//! calculations more convenient.
//! 
//! ```Test of markdown content```

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = chapter14::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Adds two to the number given.
///
/// # Examples
///
/// ```
/// let arg = 2;
/// let answer = chapter14::add_two(arg);
///
/// assert_eq!(4, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}