// ANCHOR: integration_tests
use testing::is_even;

#[test]
fn public_even_numbers() {
    assert_eq!([is_even(0), is_even(1)], [true, false]);
    assert!(is_even(4));
    assert!(!is_even(5), "5 is not even");
}

// testing::is_odd is private and cannot be called from this crate.
// ANCHOR_END: integration_tests
