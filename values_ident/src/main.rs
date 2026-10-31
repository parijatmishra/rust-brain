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

    // ANCHOR: type_annotations
    let inferred_integer = 7_i32; // Inferred as i32 from the literal.
    let annotated_integer: i32 = 7;
    assert_eq!(inferred_integer, annotated_integer);

    let inferred_boolean = true; // Inferred as bool from the initializer.
    let annotated_boolean: bool = true;
    assert_eq!(inferred_boolean, annotated_boolean);
    // ANCHOR_END: type_annotations

    // ANCHOR: later_type_constraint
    let mut number = 7; // Inferred as u8 from the later assignment.
    assert_eq!(number, 7);
    let limit: u8 = 8;
    number = limit; // `mut` permits reassignment; the type remains u8.
    assert_eq!(number, 8);
    // ANCHOR_END: later_type_constraint
}
