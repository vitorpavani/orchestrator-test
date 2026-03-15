pub fn is_palindrome(s: &str) -> bool {
    let filtered: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    let reversed: Vec<char> = filtered.iter().copied().rev().collect();
    filtered == reversed
}

pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}
