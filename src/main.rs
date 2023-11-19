fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", add(1, 2));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// test add
#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}
