use cargo02::my_add;
use pretty_assertions::assert_eq;

#[test]
fn test_my_add() {
    assert_eq!(my_add(-1, 1), 0);
    assert_eq!(my_add(2, 3), 5);
}
