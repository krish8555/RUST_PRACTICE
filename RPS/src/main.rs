// rock paper scissors game

use std::io::{self, Write};
use rand::Rng;

fn main() {
    loop {
        println!("1. Play");
        println!("2. Exit");
        print!("Enter your choice: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                let mut player_choice = String::new();
                print!("Enter your choice (rock, paper, scissors): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut player_choice).expect("Failed to read line");
                let player_choice = player_choice.trim();

                let computer_choice = rand::thread_rng().gen_range(0..3);
                let computer_choice = match computer_choice {
                    0 => "rock",
                    1 => "paper",
                    2 => "scissors",
                    _ => "error",
                };

                println!("Player: {}", player_choice);
                println!("Computer: {}", computer_choice);

                match player_choice {
                    "rock" => {
                        match computer_choice {
                            "rock" => println!("It's a tie!"),
                            "paper" => println!("Computer wins!"),
                            "scissors" => println!("Player wins!"),
                            _ => println!("Invalid choice!"),
                        }
                    }
                    "paper" => {
                        match computer_choice {
                            "rock" => println!("Player wins!"),
                            "paper" => println!("It's a tie!"),
                            "scissors" => println!("Computer wins!"),
                            _ => println!("Invalid choice!"),
                        }
                    }
                    "scissors" => {
                        match computer_choice {
                            "rock" => println!("Computer wins!"),
                            "paper" => println!("Player wins!"),
                            "scissors" => println!("It's a tie!"),
                            _ => println!("Invalid choice!"),
                        }
                    }
                    _ => println!("Invalid choice!"),
                }
            }
            2 => break,
            _ => continue,
        }
    }
}