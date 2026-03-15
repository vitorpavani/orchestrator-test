pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
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
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_divide_happy_path() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(15, 3), Some(5));
        assert_eq!(divide(-10, 2), Some(-5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), None);
        assert_eq!(divide(0, 0), None);
    }
}
