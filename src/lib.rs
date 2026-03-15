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

pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if min > max {
        return min;
    }
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

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
