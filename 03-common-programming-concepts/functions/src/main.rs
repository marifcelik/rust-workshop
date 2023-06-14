fn main() {
    println!("Hello, world!");

    another_function(12);
    print_labeled_measurement(3, 'b');

    let y = {
        let a = 5;
        a + 2
    };
    println!("the value of y is {}", y);

    let five_num = five();
    println!("method called five {five_num}");
    five();

    let y = plus_one(17);
    println!("17 plus one {y}");
}

fn another_function(x: i32) {
    println!("another function ! parameter is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
    return;
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
