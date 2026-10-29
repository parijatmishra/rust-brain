use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
    let s = "Hello, World!";
    assert_eq!(s.unicode_words().count(), 2);
}
