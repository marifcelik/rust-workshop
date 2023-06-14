fn main() {
    let number = 7;

    // error
    /* if number {

    } */

    if number != 0 {
        println!("number is not equal to zero");
    }

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let number2 = 6;

    if number2 % 4 == 0 {
        println!("number2 is divisible by 4");
    } else if number2 % 3 == 0 {
        println!("number2 is divisible by 3");
    } else if number2 % 2 == 0 {
        println!("number2 is divisible by 2");
    } else {
        println!("number2 is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num3 = if condition { 5 } else { 0 };
    println!("num3 is {num3}");

    let x;
    if true {
        x = 3;
    } else {
        x = 4;
    }

    println!("{x}");

    // LOOPS

    let mut counter = 0;

    let result = loop {
        counter += 2;

        if counter == 6 {
            break counter * 2 + 1;
        }
    };

    println!("The result is {result}");

    // loop label
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut num4 = 4;

    while num4 != 0 {
        println!("{num4} !");
        num4 -= 1;
    }
    println!("while loop end");

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", arr[index]);

        index += 1;
    }

    println!("foreach");
    for element in arr {
        println!("the value is: {element}");
    }

    println!("range");
    for num5 in (1..=3).rev() {
        println!("reversed number is {num5}")
    }
}

// TODO: implement fibonacci and fahrenheit to celsius converter
fn fibonacci(n: u32) {

}