fn main() {
    // ANCHOR: identifiers
    let item_count = 3;
    let café = 2; // Unicode letters are allowed.
    assert_eq!(item_count + café, 5);

    let _unused = 42; // A named binding; no unused-binding warning.
    let _ = 42; // No binding is created.
    // ANCHOR_END: identifiers

    // ANCHOR: raw_identifier
    let r#type = 42; // `type` is a keyword; `r#` makes it usable as a name.
    assert_eq!(r#type, 42);
    // ANCHOR_END: raw_identifier

    // ANCHOR: local_initialization
    let count = 3; // Declared and initialized together; immutable by default.
    assert_eq!(count, 3);

    let later: i32; // Declared without a value.
    later = 42; // First assignment initializes the immutable binding.
    assert_eq!(later, 42);
    // ANCHOR_END: local_initialization
}
