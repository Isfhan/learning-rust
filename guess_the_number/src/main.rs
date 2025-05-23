use rand::Rng; // rand is the random number library
// use std::cmp::Ordering; // cmp is the comparison library, Ordering is the ordering enum
use std::io; // std is the standard library, io is the input/output library 

fn main() {
    // print the welcome message
    println!("Guess the number game!");

    // generate a random number between 1 and 100
    let secret_number: u32 = rand::rng().random_range(1..=100);

    // print the secret number for debugging
    // println!("The secret number is: {secret_number}");

    // loop until the user guesses the secret number
    loop {
        // print the guess prompt
        println!("Please input your guess: ");

        // create a new string mutable variable to store the guess
        let mut guess = String::new();

        // read the guess from the user and store it in the guess variable by reference
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse the guess to a u32 number
        let guess: u32 = guess.trim().parse().expect("Please type a valid number!");

        // print the guess
        println!("You guessed: {guess}");

        // compare the guess to the secret number
        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small! Try again."),
        //     Ordering::Greater => println!("Too big! Try again."),
        //     Ordering::Equal => {
        //         println!("You win!");
        //         break;
        //     }
        // }

        if guess < secret_number {
            println!("Too small! Try again.");
        } else if guess > secret_number {
            println!("Too big! Try again.");
        } else {
            println!("You win!");
            break;
        }
    }
}
