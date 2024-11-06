use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Game!");
    let ran_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();
        println!("Enter the Number");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input!");
        println!("You Guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number between 1 - 100");
                continue;
            }
        };

        match guess.cmp(&ran_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
    }
}
