use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();

    print!("Fibonacci series up to position {}: ", input);
    for i in 0..=input {
        print!("{} ", fibo(i));
    }
    println!();
}

fn fibo(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibo(n - 1) + fibo(n - 2);
    }
}