use rand::Rng;
use std::{cmp::Ordering, io};

fn input_number() -> Result<u32, std::num::ParseIntError> {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse::<u32>()
}

pub fn run_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}

    loop {
        println!("Please input your guess.");

        let guess = match input_number() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
