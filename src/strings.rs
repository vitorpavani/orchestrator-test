/// Returns true if `s` reads the same forwards and backwards,
/// ignoring case and non-alphanumeric characters.
pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let reversed: Vec<char> = chars.iter().rev().cloned().collect();
    chars == reversed
}

/// Returns the number of whitespace-delimited words in `s`.
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}
