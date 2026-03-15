use crate::strings::{is_palindrome, word_count};

#[test]
fn test_palindrome_simple() {
    assert!(is_palindrome("racecar"));
}

#[test]
fn test_palindrome_case_insensitive() {
    assert!(is_palindrome("RaceCar"));
}

#[test]
fn test_palindrome_with_spaces_and_punctuation() {
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
}

#[test]
fn test_palindrome_empty_string() {
    assert!(is_palindrome(""));
}

#[test]
fn test_not_palindrome() {
    assert!(!is_palindrome("hello"));
}

#[test]
fn test_not_palindrome_sentence() {
    assert!(!is_palindrome("race a car"));
}

#[test]
fn test_word_count_simple() {
    assert_eq!(word_count("hello world"), 2);
}

#[test]
fn test_word_count_empty() {
    assert_eq!(word_count(""), 0);
}

#[test]
fn test_word_count_single_word() {
    assert_eq!(word_count("rust"), 1);
}

#[test]
fn test_word_count_multiple_spaces() {
    assert_eq!(word_count("  foo   bar  baz  "), 3);
}

#[test]
fn test_word_count_tabs_and_newlines() {
    assert_eq!(word_count("one\ttwo\nthree"), 3);
}
