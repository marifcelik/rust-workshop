#![allow(unused)]
fn main() {
    // variables and mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // data types
    // integer
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("parsed value is {guess}");

    // float
    let a = 2.0; // f64
    let b: f32 = 3.5; // f32

    let sum = 5 + 4;
    println!("sum is {sum}");
    let diff = 88.2 - 5.7;
    println!("diff is {diff}");
    let division1 = 13 % 4;
    println!("div1 is {division1}");
    let division2 = 37.6 / 4.5;
    println!("div2 is {division2}");
    let division3 = -9 / 2;
    println!("div3 is {division3}");
    let remainder = 83 % 9;
    println!("remainder is {remainder}");

    // boolean
    let t = true;
    let f: bool = false;

    // char
    let karakter = 'a';
    let karakter2: char = 'İ';
    let emoji = '🦀';
    println!("chars: {karakter}, {karakter2}, {emoji}");

    // tuples
    let tup1: (i32, f64, &str) = (300, 48.7, "hi");
    let (k, l, m) = tup1;
    println!("the value of m is {m}");
    let third_element = tup1.2;
    println!("third element is {third_element}");

    // array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
}
