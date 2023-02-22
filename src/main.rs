use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    println!("Guess the number!");

    // Ask for the difficulty level
    let mut level = String::new();
    println!("Please choose a difficulty level: easy (1-75), normal (1-150), hard (1-1000)");
    io::stdin()
        .read_line(&mut level)
        .expect("Failed to read line");

    let max = match level.trim() {
        "easy" => 75,
        "normal" => 150,
        "hard" => 1000,
        _ => {
            println!("Invalid input. Using default value of normal (1-150).");
            150
        }
    };

    let secret_number = rand::thread_rng().gen_range(1..=max);

    // Declare and initialize a variable for guesses
    let mut guesses = 0;

    // Ask for and store the user name
    println!("Please enter your name.");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    // Declare and initialize a variable for points
    let mut points = 100;

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
            Ordering::Less => {
                println!("Too small!");
                points -= 5;
            }
            Ordering::Greater => {
                println!("Too big!");
                points -= 5;
            }
            Ordering::Equal => {
                // Print guesses and name
                let score = points - (guesses - 1) * 5;
                println!("You win, {}! You took {} guesses and earned {} points.", name, guesses, score);

                // Add score to the scoreboard
                let mut scoreboard: HashMap<String, i32> = HashMap::new();
                scoreboard.insert(name.to_string(), score);

                // Print scoreboard
                println!("Scoreboard:");
                for (name, score) in &scoreboard {
                    println!("{}: {}", name, score);
                }

                break;
            }
        }
    }
}
