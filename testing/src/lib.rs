// ANCHOR: docs
/// Returns whether `number` is even.
///
/// # Examples
///
/// ```
/// use testing::is_even;
///
/// assert!(is_even(4));
/// assert!(!is_even(5));
/// ```
///
/// Non-integer arguments are rejected:
///
/// ```compile_fail
/// use testing::is_even;
///
/// is_even("4"); // expected u32, not &str
/// ```
pub fn is_even(number: u32) -> bool {
    !is_odd(number)
}
// ANCHOR_END: docs

// ANCHOR: helper
fn is_odd(number: u32) -> bool {
    number % 2 == 1
}
// ANCHOR_END: helper

// ANCHOR: unit_tests
#[cfg(test)]
mod tests {
    use super::is_odd; // A child module can access its parent's private items.

    #[test]
    fn private_helper() {
        assert!(is_odd(3));
        assert!(!is_odd(4));
    }
}
// ANCHOR_END: unit_tests
