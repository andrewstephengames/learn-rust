fn main () {
    let v1 = 251 + 8;
    let v2 = u8::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);
}