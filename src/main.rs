use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let string = line.expect("Can't read from stdin");
        println!("{}", string);
    }
}
