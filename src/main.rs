use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");

    loop {
        let (min, max) = get_range();

        let secret_number = rand::thread_rng().gen_range(min..=max);
        
        let mut guesses = 0;

        println!("Please enter your name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let name = name.trim();

        loop {
            println!("Please enter your guess:");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            };

            guesses += 1;

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win, {}! You took {} guesses.", name, guesses);
                    break;
                }
            }
        }

        println!("Do you want to play again? (y/n)");

        let mut play_again = String::new();
        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        if play_again.trim().to_lowercase() != "y" {
            break;
        }
    }
}

fn get_range() -> (u32, u32) {
    loop {
        println!("Please enter the minimum value of the range:");
        let mut min = String::new();
        io::stdin()
            .read_line(&mut min)
            .expect("Failed to read line");
        let min: u32 = match min.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Using default value of 1.");
                continue;
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
                continue;
            }
        };

        if min > max {
            println!("Invalid range. Please enter a valid range.");
            continue;
        }

        return (min, max);
    }
}
