use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let string = line.expect("Can't read from stdin");
        for word in string.split_whitespace() {
            let mut chars = word.chars();
            let first = chars.next().unwrap().to_string();
            let lowercase = first.to_lowercase();

            // if first character is a vowel we can just print it easily
            if "aeiouyåäö".contains(lowercase.as_str()) {
                print!("{}-hay ", word);
            } else {
                // get second character in case we need to transform it
                let mut second = chars.next().unwrap().to_string();

                // if the first character was uppercase...
                if first != lowercase {
                    // convert the second char to uppercase
                    second = second.to_uppercase();
                }

                // print the second char, the word end, and the firstay
                print!("{}{}-{}ay ",
                       second,
                       &word[(first.len() + second.len())..],
                       lowercase);
            }
        }
        println!();
    }
}
