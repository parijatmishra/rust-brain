use unicode_segmentation::UnicodeSegmentation;

// ANCHOR: lib-core
pub fn my_add(a: i32, b: i32) -> i32 {
    a + b
}
// ANCHOR_END: lib-core

// ANCHOR: lib-words
pub fn word_count(s: &str) -> usize {
    s.unicode_words().count()
}
// ANCHOR_END: lib-words

// ANCHOR: lib-tests
#[cfg(test)]
mod test_my_add {
    use super::my_add;
    #[test]
    fn test_my_add() {
        assert_eq!(my_add(-1, 1), 0);
    }
}
// ANCHOR_END: lib-tests
