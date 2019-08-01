extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut count = 0; // Measure the number of times to resolve

    loop {
        println!("Please input your guess. [\"quit\" to exit game.]");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        if guess.trim().to_lowercase() == "quit" {
            println!("Quit game.");
            break;
        };

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{} is too small!", guess);
                count += 1
            }
            Ordering::Greater => {
                println!("{} is too big!", guess);
                count += 1;
            }
            Ordering::Equal => {
                println!("You win!");
                count += 1;
                println!("You tried {} times to win this game.", count);
                break;
            }
        }

        println!();
    }
}
