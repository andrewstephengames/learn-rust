fn main () {
    assert!(1u32+2 == 3u32);

    assert!(1i32-2 == -1i32);
    assert!(1u8 as i8-2 == -1);

    assert!(3*50 as u8 == 150 as u8);

    assert!((9.6 as i8) / (3.2 as i8) == 3.0 as i8);
    
    assert!(24%5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101u32);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101u32);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101u32);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}