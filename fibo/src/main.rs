use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();
    let result = fibo(input);
    println!("Fibonacci number at position {} is {}", input, result);

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