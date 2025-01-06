use std::io::Read;
use std::env;
use std::fs::File;
fn main() {
let args = env::args().collect::<Vec<String>>();
if args.len() != 2 {
    println!("Usage: word_count <filename>");
    return;
}
let file_path = &args[1];
println!("Reading file: {}", file_path);
let mut file = match File::open(file_path) {
    Ok(file) => file,
    Err(e) => {
        println!("Error: {}", e);
        return;
    }
};
let mut contents = String::new();
if let Err(e) = file.read_to_string(&mut contents) {
    println!("Error: {}", e);
    return;
}
let word_count = count_words(&contents);
println!("Word count: {}", word_count);
let line_count = count_lines(&contents);
println!("Line count: {}", line_count);
let char_count = count_chars(&contents);
println!("Character count: {}", char_count);


}


fn count_words(contents: &str) -> usize {
contents.split_whitespace().count()
}

fn count_lines(contents: &str) -> usize {
contents.lines().count()
}
fn count_chars(contents: &str) -> usize {
contents.chars().count()
}

