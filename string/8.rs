fn main () {
    let s = "hello world".to_string();
    //let s = "hello world";
    // let s1: &str = s;
    let s1: &str = &s;
    println!("success");
}