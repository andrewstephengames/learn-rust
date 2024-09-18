fn main () {
    let v: () = ();
    assert_eq!(v, implicitly_ret_unit());
    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

fn explicity_ret_unit() -> () {
    println!("I will return a ()");
}