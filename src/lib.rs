pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
    fn test_divide_success() {
        let result = divide(10, 2);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10, 0);
        assert_eq!(result, None);
    }
}
