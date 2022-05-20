fn main() {
    println!("Hello, world!");
    // Deref
    let s1 = &5;
    let s2 = Box::new(5);
    assert_eq!(5, *s1);
    assert_eq!(5, *s2);
}
