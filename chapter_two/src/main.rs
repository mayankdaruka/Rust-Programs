use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("{}", secret_number);

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    // let int_guess : u32 = guess.trim().parse().expect("Please type a number!");

    let int_guess : u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0, // use _ to explicitly ignore each field, and .. to ignore all fields
    };

    match int_guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
}
