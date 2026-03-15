use crate::strings::{is_palindrome, word_count};

#[test]
fn test_is_palindrome_simple() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("level"));
    assert!(is_palindrome("madam"));
}

#[test]
fn test_is_palindrome_case_insensitive() {
    assert!(is_palindrome("Racecar"));
    assert!(is_palindrome("RaceCar"));
    assert!(is_palindrome("Madam"));
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
    assert!(!is_palindrome("openai"));
}

#[test]
fn test_is_palindrome_edge_cases() {
    assert!(is_palindrome(""));
    assert!(is_palindrome("a"));
    assert!(is_palindrome("  "));
}

#[test]
fn test_word_count_basic() {
    assert_eq!(word_count("hello world"), 2);
    assert_eq!(word_count("one two three"), 3);
    assert_eq!(word_count("single"), 1);
}

#[test]
fn test_word_count_extra_whitespace() {
    assert_eq!(word_count("  hello   world  "), 2);
    assert_eq!(word_count("a  b  c"), 3);
    assert_eq!(word_count("\tone\ttwo\n"), 2);
}

#[test]
fn test_word_count_empty_and_whitespace_only() {
    assert_eq!(word_count(""), 0);
    assert_eq!(word_count("   "), 0);
    assert_eq!(word_count("\n\t"), 0);
}

#[test]
fn test_word_count_single_word_variants() {
    assert_eq!(word_count("rust"), 1);
    assert_eq!(word_count("  rust  "), 1);
    assert_eq!(word_count("HELLO"), 1);
}
