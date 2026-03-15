pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn power(base: i32, exp: u32) -> Option<i32> {
    if exp == 0 {
        return Some(1);
    }

    let mut result: i32 = 1;
    for _ in 0..exp {
        result = result.checked_mul(base)?;
    }
    Some(result)
}

#[cfg(test)]
mod tests;
