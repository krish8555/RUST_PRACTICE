use std::io;

fn main() {
    println!("simple calculator");
    println!("avialable operations: +, -, *, /");
    println!("enter a expression: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    let num1: f64 = tokens[0].parse().expect("failed to parse number");
    let op: &str = tokens[1];
    let num2: f64 = tokens[2].parse().expect("failed to parse number");

    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("invalid operator");
            return;
        }
    };
    println!("result: {}", result);
    



    


}
