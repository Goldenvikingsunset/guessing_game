use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Ask for the minimum and maximum values of the range
    println!("Please enter the minimum value of the range:");
    let mut min = String::new();
    io::stdin()
        .read_line(&mut min)
        .expect("Failed to read line");
    let min: u32 = match min.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default value of 1.");
            1
        }
    };

    println!("Please enter the maximum value of the range:");
    let mut max = String::new();
    io::stdin()
        .read_line(&mut max)
        .expect("Failed to read line");
    let max: u32 = match max.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default value of 100.");
            100
        }
    };

    let secret_number = rand::thread_rng().gen_range(min..=max);
    
    // Declare and initialize a variable for guesses
    let mut guesses = 0;

    // Ask for and store the user name
    println!("Please enter your name.");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Increment guesses by one
        guesses += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // Print guesses and name
                println!("You win, {}! You took {} guesses.", name, guesses);
                break;
            }
        }
    }
}
