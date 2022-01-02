// Apparently rust style is to indent with 4 spaces, not a tab?

use std::io;

fn main() {
    println!("Hello, world!");

    println!("Guess the number!");

    // Use let to create a variable
    // Use mut before a variable name to make the variable mutable
    let mut guess = String::new();

    // References are immutable by default, so have to write "&mut guess" instead of "&guess"
    // Expect is used to handle io::Result if it is an error
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);
}