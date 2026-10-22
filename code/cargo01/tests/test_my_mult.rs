use cargo01::my_mult;

#[test]
fn test_my_mult01() {
    assert_eq!(my_mult(0, 1), 0);
    assert_eq!(my_mult(2, 3), 6);
}
