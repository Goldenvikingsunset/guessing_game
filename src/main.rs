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

    // Initialize variables for scoreboard
    let mut games_played = 0;
    let mut scoreboard: HashMap<String, (i32, i32, i32)> = HashMap::new();

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=max);

        // Declare and initialize a variable for guesses
        let mut guesses = 0;

        // Ask for and store the user name
        println!("Please enter your name, or type 'q' to quit.");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        if name.trim() == "q" {
            break;
        }

        let name = name.trim();

        // Declare and initialize a variable for points
        let mut points = 100;

        loop {
            println!("Please input your guess, or type 'quit' to quit.");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            if guess.trim() == "quit" {
                break;
            }

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

                    // Update scoreboard
                    games_played += 1;
                    let mut player_score = scoreboard.entry(name.to_string()).or_insert((0, 0, 0));
                    player_score.0 += 1;
                    player_score.1 += guesses as i32;
                    player_score.2 += score;

                    // Print scoreboard
                    println!("Scoreboard:");
                    for (name, (games, guesses, score)) in &scoreboard {
                        println!("{}: Games played: {}, Guesses: {}, Score: {}", name, games, guesses, score);
                    }

                    break;
                }
            }
        }
    }
    println!("Thanks for playing! Games played: {}", games_played);
}