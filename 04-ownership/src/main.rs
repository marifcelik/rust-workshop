#![allow(unused)]
fn main() {
    let a = Box::new([0; 1_000_000]);
    let b = a;
    // value moved to b and a deallocated
    // println!("{}", a.len());

    // collection uses boxes
    let first = String::from("junior");
    let full = add_suffix(first); // first points to deallocated

    // undefined behavior ! - variable cannot be used after being moved
    // println!("{full} originally is {first}");
    println!("{full}");

    // clone
    let second = String::from("senior");
    let second_clone = second.clone();
    let full2 = add_suffix(second_clone);
    println!("{full2} originally is {second}"); // now it works !

    let m1 = String::from("hi");
    let m2 = String::from("jack");
    greet(&m1, &m2);
    let s = format!("{} {} is still available", m1, m2);
    println!("{s}");

    // dereferencing a pointer
    let mut x = Box::new(5);
    let mut c = *x; // c = 5
    *x += 1;
    c += 5;

    let r1 = &x; // points to x on the stack - r1 = x
    let d = **r1;
    let r2 = &*x; // points to value of x's reference, to heap value
    let mut e = *r2; // direct data
    e += 2;

    println!("x\t{x}");
    println!("c\t{c}");
    println!("r1\t{r1}");
    println!("d\t{d}");
    println!("r2\t{r2}");
    println!("e\t{e}");

    let z = Box::new(-1);
    let z_abs1 = i32::abs(*z);
    let z_abs2 = z.abs();
    assert_eq!(z_abs1, z_abs2);

    let z_r = &x;
    let z_r_abs1 = i32::abs(**z_r);
    let z_r_abs2 = z_r.abs();
    assert_eq!(z_r_abs1, z_r_abs2);

    let s = String::from("hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);

    let deneme = Box::new(0);
    let denemey = Box::new(&deneme);

    // mutation
    let mut vec: Vec<i32> = vec![1, 2, 3];
    // let vec_num = &vec[2]; // undefined behavior, we cant do that !
    vec.push(4); // vec has to create new array, copy old values and deallocate the original vec

    for i in 0..vec.len() {
        print!("{} ", vec[i]);
    }
    println!("");

    let vec_num: &mut i32 = &mut vec[2];
    *vec_num += 1;
    println!("now the third element of vec is {}", *vec_num);
    println!("vec is {:?}", vec);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" senior");
    name
}

fn greet(g1: &String, g2: &String) {
    println!("{g1} {g2}");
}
