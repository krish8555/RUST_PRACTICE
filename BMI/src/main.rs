use std::io;
fn main() {
    println!("Enter your weight in kg: ");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Failed to read line");
    let weight: f32 = weight.trim().parse().expect("Please type a number!");

    println!("Enter your height in cm: ");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height: f32 = height.trim().parse().expect("Please type a number!");

    if weight <= 0.0 || height <= 0.0 {
        println!("Invalid input");
        return;
    }

    let height = height / 100.0;
    let bmi = weight / (height * height);
    println!("Your BMI is: {}", bmi);
    if bmi < 18.5 {
        println!("Underweight");
    } else if bmi < 24.9 {
        println!("Normal weight");
    } else if bmi < 29.9 {
        println!("Overweight");
    } else {
        println!("Obesity");
    }
}
