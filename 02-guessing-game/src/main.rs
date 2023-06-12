use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("guess game !");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("please enter your guess : ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("not a valid number !");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("correct answer !");
                break;
            }
        }
    }
}
