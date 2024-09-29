fn main () {
    let s = "hello, world";
    //let s = "hello, world".to_string();
    greetings(String::from(s))
}

fn greetings (s: String) {
    println!("{}", s)
}