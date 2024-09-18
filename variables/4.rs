fn main () {
    define_x();
}

fn define_x() {
    let x: &str = "hello"; // let x: String = "hello".to_string();
    println!("{}, world", x);
}