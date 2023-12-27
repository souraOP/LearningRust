use std::io;  // brings the input output library from the standard library (std)
use rand::Rng;   // Rng trait defines that random number generators implement and this trait must be scope to use it.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please enter your guess: ");

        let mut guess = String::new();     // introduces an mutable variable 'guess'
        // ' :: ' means an associated function that is implemented on a type
        // ' new ' function that makes a new value of some kind
        // String::new creates a new empty string



        // Rust has strong, static type system.
        // Rust default to i32, 32 bit number
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You have guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number is too small"),
            Ordering::Greater => println!("Number is too large"),
            Ordering::Equal => {
                println!("You won!!");
                break;
            },
        }
    }


    
}

