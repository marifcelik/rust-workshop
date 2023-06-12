use std::io::{self, Write};

fn main() {
    println!("guess game !");
    print!("please enter your guess : ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    guess = guess.trim().to_string();

    print!("you are pressed {} !", guess);
}
