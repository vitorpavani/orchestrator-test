pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_is_even_positive_even() {
        assert!(is_even(2));
        assert!(is_even(4));
        assert!(is_even(100));
    }

    #[test]
    fn test_is_even_positive_odd() {
        assert!(!is_even(1));
        assert!(!is_even(3));
        assert!(!is_even(99));
    }

    #[test]
    fn test_is_even_zero() {
        assert!(is_even(0));
    }

    #[test]
    fn test_is_even_negative_even() {
        assert!(is_even(-2));
        assert!(is_even(-4));
        assert!(is_even(-100));
    }

    #[test]
    fn test_is_even_negative_odd() {
        assert!(!is_even(-1));
        assert!(!is_even(-3));
        assert!(!is_even(-99));
    }
}
