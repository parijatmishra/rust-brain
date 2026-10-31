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

    // ANCHOR: mutable_binding
    let mut score = 10;
    assert_eq!(score, 10);
    score = 20; // Reassignment changes the value, not the binding's type.
    assert_eq!(score, 20);
    // ANCHOR_END: mutable_binding

    // ANCHOR: shadowing
    let value = 3;
    let value = value + 2; // A new binding; the initializer sees the old value.
    assert_eq!(value, 5);
    let value = value > 4; // Another binding, now of type bool.
    assert!(value);
    // ANCHOR_END: shadowing

    // ANCHOR: shadow_mutability
    let mut level = 1;
    level += 1;
    let level = level; // The new binding is immutable.
    assert_eq!(level, 2);
    // ANCHOR_END: shadow_mutability

    // ANCHOR: nested_scope
    let threshold = 10;
    {
        let threshold = 20; // Shadows the outer binding inside this block.
        assert_eq!(threshold, 20);
    }
    assert_eq!(threshold, 10); // The outer binding is visible again.
    // ANCHOR_END: nested_scope

    // ANCHOR: constant_value
    const DEFAULT_LIMIT: u32 = 2 * 5; // Constant expression, evaluated at compile time.
    let first_limit = DEFAULT_LIMIT;
    let second_limit = DEFAULT_LIMIT + 1;
    assert_eq!(first_limit, 10);
    assert_eq!(second_limit, 11);
    // ANCHOR_END: constant_value

    // ANCHOR: constant_and_runtime
    fn add_step(input: u32) -> u32 {
        const STEP: u32 = 2 * 5; // Constant expression.
        let total = input + STEP; // Immutable local using a runtime parameter.
        total
    }
    assert_eq!(add_step(3), 13);
    // ANCHOR_END: constant_and_runtime

    // ANCHOR: constant_shadowing
    const LIMIT: u32 = 10;
    {
        const LIMIT: u32 = 20; // Shadows the outer constant in this block.
        assert_eq!(LIMIT, 20);
    }
    assert_eq!(LIMIT, 10); // The outer constant is visible again.
    // ANCHOR_END: constant_shadowing
}
