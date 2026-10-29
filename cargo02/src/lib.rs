pub fn my_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test_my_add {
    use super::my_add;
    #[test]
    fn test_my_add() {
        assert_eq!(my_add(-1, 1), 0);
    }
}
