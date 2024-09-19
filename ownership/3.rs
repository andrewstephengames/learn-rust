fn main () {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from ("Hello world");
    let _s = s.clone().into_bytes();
    s
}