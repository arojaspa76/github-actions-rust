use trust::add;
use trust::div;
use trust::mul;
use trust::sub;

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add(-1, -2), -3);
    assert_eq!(add(0, 0), 0);
}
#[test]
fn test_sub() {
    assert_eq!(sub(2, 1), 1);
    assert_eq!(sub(-1, -2), 1);
    assert_eq!(sub(0, 0), 0);
}
#[test]
fn test_mul() {
    assert_eq!(mul(2, 3), 6);
    assert_eq!(mul(-1, -2), 2);
    assert_eq!(mul(0, 5), 0);
}
#[test]
fn test_div() {
    assert_eq!(div(6, 2).unwrap(), 3);
    assert_eq!(div(-6, -2).unwrap(), 3);
    assert_eq!(div(0, 1).unwrap(), 0);

    // Test division by zero
    let result = div(1, 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Cannot divide by zero");
}
