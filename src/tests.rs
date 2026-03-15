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

#[test]
fn test_subtract() {
    assert_eq!(subtract(5, 3), 2);
}

#[test]
fn test_power_zero_exponent() {
    assert_eq!(power(5, 0), Some(1));
    assert_eq!(power(0, 0), Some(1));
    assert_eq!(power(-3, 0), Some(1));
}

#[test]
fn test_power_base_zero() {
    assert_eq!(power(0, 1), Some(0));
    assert_eq!(power(0, 5), Some(0));
}

#[test]
fn test_power_positive_result() {
    assert_eq!(power(2, 3), Some(8));
    assert_eq!(power(3, 4), Some(81));
    assert_eq!(power(-2, 3), Some(-8));
    assert_eq!(power(-2, 2), Some(4));
    assert_eq!(power(10, 2), Some(100));
}

#[test]
fn test_power_overflow() {
    assert_eq!(power(2, 31), None);
    assert_eq!(power(1000, 10), None);
    assert_eq!(power(i32::MAX, 2), None);
}
