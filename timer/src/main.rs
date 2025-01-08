use std::{thread, time::Duration};
use std::io::{self, Write};

fn main() {
    loop {
        println!("1. Set timer");
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
                let mut duration = String::new();
                print!("Enter duration in seconds: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut duration).expect("Failed to read line");
                let duration: u64 = match duration.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let mut message = String::new();
                print!("Enter message to display when timer expires: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut message).expect("Failed to read line");

                println!("Timer started for {} seconds", duration);
                thread::sleep(Duration::from_secs(duration));
                println!("Timer expired! Message: {}", message.trim());
            }
            2 => break,
            _ => continue,
        }
    }
}