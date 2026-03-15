/// Adds two unsigned 64-bit integers.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Returns `true` if the integer is even, `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::is_even;
/// assert!(is_even(4));
/// assert!(!is_even(7));
/// ```
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Divides `a` by `b`, returning `None` if `b` is zero.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::divide;
/// assert_eq!(divide(10, 2), Some(5));
/// assert_eq!(divide(5, 0), None);
/// ```
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

/// Multiplies two 32-bit integers.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::multiply;
/// assert_eq!(multiply(3, 4), 12);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Subtracts `b` from `a`.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::subtract;
/// assert_eq!(subtract(10, 3), 7);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Computes the greatest common divisor of two unsigned 64-bit integers
/// using the Euclidean algorithm.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::gcd;
/// assert_eq!(gcd(12, 8), 4);
/// assert_eq!(gcd(7, 0), 7);
/// ```
pub fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

#[cfg(test)]
mod tests;
