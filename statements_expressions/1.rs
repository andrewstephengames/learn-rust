fn main () {
    let v = {
        let x = 1;
        x+2
    };
    println!("{}", v);
    assert_eq!(v, 3);
    println!("Success");
}