// Import the necessary libraries
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Print the starting message
    println!("Guess the number!");

    // Generate a random number
    const MIN_NUMBER: u32 = 1;
    const MAX_NUMBER: u32 = 100;
    let secret_number = rand::thread_rng().gen_range(MIN_NUMBER..=MAX_NUMBER);

    // Declare and initialize a variable for guesses
    let mut guesses = 0;

    // Ask for and store the user name
    println!("Please enter your name.");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    // Loop until the user guesses the number
    loop {
        // Ask for the user's guess
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Increment the number of guesses
        guesses += 1;

        // Print the user's guess
        println!("You guessed: {}", guess);

        // Check if the user's guess is correct
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Print the final message
                println!("You win, {}! You took {} guesses.", name, guesses);
                break;
            }
        }
    }
}
