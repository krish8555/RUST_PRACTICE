use std::io;
fn main() {
    println!("Enter a word: ");
    let mut word = String::new();
    io::stdin().read_line(&mut word).expect("Failed to read line");
    let word = word.trim();
    if is_palindrome(word) {
        println!("{} is a palindrome", word);
    } else {
        println!("{} is not a palindrome", word);
    }
}
fn clean(word: &str) -> String {
    word.chars().filter(|c| c.is_alphanumeric()).collect()
}
fn is_palindrome(word: &str) -> bool {
    let word = clean(word);
    let reversed_word: String = word.chars().rev().collect();
    word == reversed_word
}