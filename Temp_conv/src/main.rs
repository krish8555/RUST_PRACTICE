use std::io;
fn main() {
    println!("Temprature converter ");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Enter your choice: ");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number!");
    if choice == 1 {
        println!("Enter the temperature in Celsius: ");
        let mut celsius = String::new();
        io::stdin().read_line(&mut celsius).expect("Failed to read line");
        let celsius: f32 = celsius.trim().parse().expect("Please type a number!");
        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
        println!("The temperature in Fahrenheit is: {}", fahrenheit);
    } else if choice == 2 {
        println!("Enter the temperature in Fahrenheit: ");
        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
        let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please type a number!");
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("The temperature in Celsius is: {}", celsius);
    } else {
        println!("Invalid choice");
    }

}
