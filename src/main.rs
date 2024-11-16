use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the guessing game!");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("SHHHHH! The secret number is: {} :)", secret_number);

        println!("Please guess a number between 1 and 100:");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => {
                if guess.eq("\r\n") {
                    break;
                }
                
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
