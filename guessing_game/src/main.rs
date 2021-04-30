use std::io;
use std::cmp::Ordering;
use rand::Rng;

const MAXIMUM_TRIES: i8 = 6;

fn main() {
    println!("*****************");
    println!("* Guessing Game *");
    println!("*****************");

    let secret_number = rand::thread_rng().gen_range(0..101);
    
    let mut trial_number = 1;

    loop {
        println!("Guess a number, trial {} of {}", trial_number, MAXIMUM_TRIES);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Err reading your guess");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { 
                println!("Please, enter a valid number");
                continue;
            }
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                trial_number += 1;
            },
            Ordering::Greater => { 
                println!("Too big!"); 
                trial_number += 1;
            },
            Ordering::Equal => {
                println!("You win!");
                std::process::exit(1);
            }
        }

        if trial_number > MAXIMUM_TRIES {
            eprintln!("You loose, maximum tries exceeded. The secret number was: {}", secret_number);
            std::process::exit(0);
        }
    }

}
