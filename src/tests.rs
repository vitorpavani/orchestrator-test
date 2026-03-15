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
fn test_clamp_value_below_min() {
    assert_eq!(clamp(-5, 0, 10), 0);
}

#[test]
fn test_clamp_value_above_max() {
    assert_eq!(clamp(15, 0, 10), 10);
}

#[test]
fn test_clamp_value_in_range() {
    assert_eq!(clamp(5, 0, 10), 5);
}

#[test]
fn test_clamp_min_equals_max() {
    assert_eq!(clamp(5, 7, 7), 7);
    assert_eq!(clamp(7, 7, 7), 7);
}

#[test]
fn test_clamp_min_greater_than_max() {
    assert_eq!(clamp(5, 10, 0), 10);
}
