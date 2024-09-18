fn main () {
    println!("i8::MAX={}", i8::MAX);
    println!("u8::MAX={}", u8::MAX);
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
}