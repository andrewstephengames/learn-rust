fn main () {
    let s = String::from("Hello world");
    print_str (&s);
    println!("{}", s);
}

fn print_str(s: &String) {
    println!("{}", s);
}