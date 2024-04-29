/// Adds two 32-bit integers together (sum)
///
/// # Examples
///
/// ```rust
/// use template::math::add_two_i32;
///
/// let x: i32 = 3;
/// let y: i32 = 5;
/// let z = add_two_i32(x, y);
/// assert_eq!(8, z);
/// ```
pub fn add_two_i32(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_i32() {
        assert_eq!(8, add_two_i32(3, 5));
    }
}
