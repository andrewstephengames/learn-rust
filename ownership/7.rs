fn main () {
    let x = Box::new(5);
    let y: &mut i32 = &mut 42;
    *y = 4;
    assert_eq!(*x, 5);
    println!("Success!");
}