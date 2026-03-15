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
fn test_gcd_zero_zero() {
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn test_gcd_zero_n() {
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(7, 0), 7);
}

#[test]
fn test_gcd_coprime() {
    assert_eq!(gcd(13, 17), 1);
    assert_eq!(gcd(25, 9), 1);
}

#[test]
fn test_gcd_common_divisor() {
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(100, 50), 50);
    assert_eq!(gcd(54, 24), 6);
}

#[test]
fn test_clamp_below_range() {
    assert_eq!(clamp(-5, 0, 10), 0);
}

#[test]
fn test_clamp_above_range() {
    assert_eq!(clamp(15, 0, 10), 10);
}

#[test]
fn test_clamp_in_range() {
    assert_eq!(clamp(5, 0, 10), 5);
}

#[test]
fn test_clamp_min_equals_max() {
    assert_eq!(clamp(3, 7, 7), 7);
    assert_eq!(clamp(7, 7, 7), 7);
    assert_eq!(clamp(10, 7, 7), 7);
}

#[test]
fn test_clamp_min_greater_than_max() {
    assert_eq!(clamp(5, 10, 5), 10);
}
