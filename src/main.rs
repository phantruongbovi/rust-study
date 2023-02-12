fn main() {
    // variable
    let mut x: i32 = 10;
    print!("x = {}", x);
    x = 20;
    print!("x = {}", x);

    const HANG_SO: u32 = 10_000_000;
    print!("HANG SO = {}", HANG_SO);

    // shadowing
    let x: i32 = 10;
    print!("x = {}", x);
    let x: &str = "truong";
    print!("x = {}", x);

    // Data types

    // Integer
    let x: u32 = 1000000000;
    let b = 0xff; // Hex 255
    let c = 0o77; // Octal 63
    let d = 0b11_00; // binary 240
    let e = b'A'; // Byte (u8 only)

    // Float
    let f = 2.0;
    let g = 3.0;

    // Boolean
    let t = true;
    let g: bool = false;

    //String
    let c = 'z';

    // Tuple
    let tup = ("hello", 100_100);
    println!("tuple = {:?}", tup);
    let (_string, _integer) = tup;
    println!("{}", _string);
    println!("{}", _integer);

    // Array
    let number = [100, 200, 300];
    let get_number = number[1];
    println!("{}", get_number);
    println!("{:?}", number);

    for i in number.iter() {
        println!("{}", i);
    }

    let hashing = [0; 32];
    for i in hashing.iter() {
        print!("{}", i);
    }
}
