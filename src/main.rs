use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let string = line.expect("Can't read from stdin");
        for word in string.split_whitespace() {
            if "aeiouy".contains(&word[..1]) {
                print!("{}-hay ", word);
            } else {
                print!("{}-{}ay", &word[1..], &word[..1]);
            }
        }
        println!();
    }
}
