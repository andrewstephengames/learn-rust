fn main () {
    let x = 5;
    assert_eq!("u32".to_string(), type_of(&(x as u32)));
    println!("Success!");
}

fn type_of<T> (_: &T) -> String {
    return format!("{}", std::any::type_name::<T>());
}