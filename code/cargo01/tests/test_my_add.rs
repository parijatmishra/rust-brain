use cargo01::my_add;

#[test]
fn test_my_add01() {
    assert_eq!(my_add(-1, 1), 0);
    assert_eq!(my_add(2, 3), 5);
}
