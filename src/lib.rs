pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
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
    fn test_is_even_with_even_number() {
        assert!(is_even(4));
    }

    #[test]
    fn test_is_even_with_odd_number() {
        assert!(!is_even(3));
    }

    #[test]
    fn test_is_even_with_zero() {
        assert!(is_even(0));
    }

    #[test]
    fn test_is_even_with_negative_even() {
        assert!(is_even(-2));
    }

    #[test]
    fn test_is_even_with_negative_odd() {
        assert!(!is_even(-3));
    }

    #[test]
    fn test_divide_normal() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(7, 3), Some(2));
        assert_eq!(divide(-10, 2), Some(-5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), None);
        assert_eq!(divide(0, 0), None);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }
}
