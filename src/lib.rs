/// Adds two unsigned 64-bit integers together.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::add;
///
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(0, 0), 0);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Returns `true` if the given integer is even, `false` otherwise.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::is_even;
///
/// assert!(is_even(4));
/// assert!(!is_even(7));
/// assert!(is_even(0));
/// ```
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Divides `a` by `b`, returning `Some(quotient)` or `None` if `b` is zero.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::divide;
///
/// assert_eq!(divide(10, 2), Some(5));
/// assert_eq!(divide(7, 0), None);
/// ```
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

/// Multiplies two integers and returns the product.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::multiply;
///
/// assert_eq!(multiply(3, 4), 12);
/// assert_eq!(multiply(-2, 5), -10);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Subtracts `b` from `a` and returns the result.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::subtract;
///
/// assert_eq!(subtract(10, 3), 7);
/// assert_eq!(subtract(0, 5), -5);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Computes the greatest common divisor (GCD) of two unsigned 64-bit integers
/// using the Euclidean algorithm.
///
/// # Examples
///
/// ```rust
/// use orchestrator_test::gcd;
///
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(0, 5), 5);
/// assert_eq!(gcd(7, 7), 7);
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
