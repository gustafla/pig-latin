use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let string = line.expect("Can't read from stdin");
        for word in string.split_whitespace() {
            let first = word.chars().next().unwrap();
            let lowercase: String = first.to_lowercase().collect();
            if "aeiouyåäö".contains(lowercase.as_str()) {
                print!("{}-hay ", word);
            } else {
                print!("{}-{}ay ", &word[first.len_utf8()..], lowercase);
            }
        }
        println!();
    }
}
