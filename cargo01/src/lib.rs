// ANCHOR: lib-core
pub fn my_add(a: i32, b: i32) -> i32 {
    a + b
}
// ANCHOR_END: lib-core

// ANCHOR: lib-tests
#[cfg(test)]
mod test_my_add {
    use super::my_add;
    #[test]
    fn test_my_add() {
        assert_eq!(my_add(-1, 1), 0);
    }
}
// ANCHOR-END: lib-tests
