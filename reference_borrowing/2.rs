fn main () {
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
    println!("Success!");
}