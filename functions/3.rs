fn main () {
    never_return();
    println!("Failed");
}

fn never_return() -> ! {
    //alert!("deez nuts");
    unimplemented!("deez nuts");
}