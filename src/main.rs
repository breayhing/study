use std::io;
use study::acronym;

fn main() {
    println!("Enter a long name to generate its acronym:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let acronym = acronym(&input.trim());

    println!("Long name: {}",input.trim());
    println!("Acronym:  {}",acronym);
}
