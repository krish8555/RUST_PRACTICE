use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Available operations: +, -, *, /");
    println!("Enter an expression:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim and split the input into tokens
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    // Check if the input is correctly formatted
    if tokens.len() != 3 {
        println!("Invalid expression format. Expected: <num1> <operator> <num2>");
        return;
    }

    // Parse the first number
    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Failed to parse number: {}", tokens[0]);
            return;
        }
    };

    // Parse the operator
    let op: &str = tokens[1];

    // Parse the second number
    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Failed to parse number: {}", tokens[2]);
            return;
        }
    };

    // Perform the calculation based on the operator
    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("Invalid operator: {}", op);
            return;
        }
    };

    // Display the result
    println!("Result: {}", result);
}
