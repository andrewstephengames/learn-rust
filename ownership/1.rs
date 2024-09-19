fn main() {
    let mut x = String::from("Hello world");
    let y = x.clone();
    //let y = &x;
    println!("{}, {}", x, y);
    // println!("{x}");
    // let y = &mut x;
    // print!("{}", y);
}