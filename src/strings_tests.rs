use crate::strings::{is_palindrome, word_count};

#[test]
fn test_is_palindrome_simple() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("madam"));
    assert!(is_palindrome("level"));
}

#[test]
fn test_is_palindrome_case_insensitive() {
    assert!(is_palindrome("Racecar"));
    assert!(is_palindrome("MadaM"));
    assert!(is_palindrome("RaceCar"));
}

#[test]
fn test_is_palindrome_ignores_non_alphanumeric() {
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome("Was it a car or a cat I saw?"));
    assert!(is_palindrome("No 'x' in Nixon"));
}

#[test]
fn test_is_palindrome_not_palindrome() {
    assert!(!is_palindrome("hello"));
    assert!(!is_palindrome("world"));
    assert!(!is_palindrome("rust"));
}

#[test]
fn test_is_palindrome_empty_and_single() {
    assert!(is_palindrome(""));
    assert!(is_palindrome("a"));
    assert!(is_palindrome("Z"));
}

#[test]
fn test_word_count_basic() {
    assert_eq!(word_count("hello world"), 2);
    assert_eq!(word_count("one two three"), 3);
    assert_eq!(word_count("single"), 1);
}

#[test]
fn test_word_count_multiple_spaces() {
    assert_eq!(word_count("  hello   world  "), 2);
    assert_eq!(word_count("a  b   c    d"), 4);
}

#[test]
fn test_word_count_empty_and_whitespace() {
    assert_eq!(word_count(""), 0);
    assert_eq!(word_count("   "), 0);
    assert_eq!(word_count("\t\n"), 0);
}

#[test]
fn test_word_count_newlines_and_tabs() {
    assert_eq!(word_count("hello\tworld"), 2);
    assert_eq!(word_count("line1\nline2\nline3"), 3);
    assert_eq!(word_count("a\tb\nc"), 3);
}
