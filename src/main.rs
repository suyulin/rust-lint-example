fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", add(1, 2));
    println!("1 - 2 = {}", sub(1, 2));
}
fn sub(a: i32, b: i32) -> i32 {
    a - b
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// test add
#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
// test sub
#[test]
fn test_sub() {
    assert_eq!(sub(1, 2), -1);
}
