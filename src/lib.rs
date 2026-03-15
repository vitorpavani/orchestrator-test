//! Math utilities library
//!
//! Provides basic mathematical operations.
//!
//! # Examples
//!
//! ```
//! use orchestrator_test::add;
//!
//! let result = add(2, 3);
//! assert_eq!(result, 5);
//! ```

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
