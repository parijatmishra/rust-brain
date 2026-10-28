use unicode_segmentation::UnicodeSegmentation;

pub fn my_add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn word_count(s: &str) -> usize {
    s.unicode_words().count()
}

#[cfg(test)]
mod test_my_add {
    use super::my_add;
    #[test]
    fn test_my_add() {
        assert_eq!(my_add(-1, 1), 0);
    }
}

#[cfg(test)]
mod test_word_count {
    use super::word_count;
    #[test]
    fn test_word_count() {
        assert_eq!(word_count("Hello World"), 2);
    }
}
