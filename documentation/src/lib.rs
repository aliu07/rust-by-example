// '//!' documents the the item that contains this comment rather than the item
// following the comment like with '///'. Use '//!' to give descriptions to modules
// and crates (i.e. the containers).

//! # Documentation
//!
//! `documentation` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = documentation::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_test() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
