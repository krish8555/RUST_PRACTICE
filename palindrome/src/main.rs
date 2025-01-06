use std::io;
fn main() {
    println!("Enter a word: ");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim();

    let mut reversed_word = String::new();
    for c in word.chars().rev() {
        reversed_word.push(c);
    }

    if word == reversed_word {
        println!("{} is a palindrome", word);
    } else {
        println!("{} is not a palindrome", word);
    }
}