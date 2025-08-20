use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    loop {
    println!("Guess a number!");
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret_number is {secret_number}");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read_line");

    println!("You guessed {guess}");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("A number was expected!");
            continue;
        },
    };

    match guess.cmp(&secret_number) {
        Ordering::Less =>println!("Too small!"),
        Ordering::Greater =>println!("Too big!"),
        Ordering::Equal =>println!("Big win!"),
    }

    }
}
