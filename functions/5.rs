fn main () {
    let b = false;
    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("We have no value for `false`, but we can panic");
        }
    };
    
    println!("Exercise failed if printing out this line!");
}