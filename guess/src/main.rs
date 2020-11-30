extern crate rand;

use std::io;
use rand::random;

/// Guess documentation
/// ```rust
///     println!("Hello, welcome to the guessing game!");
/// ```

fn get_guess () -> u8 {
    loop {
        let mut guess : String = String::new();

        println!("Input your guess, it should be a number:");

        io::stdin().read_line(&mut guess).expect("Couldn't read from stdin");

        println!("You guessed: {}", guess);

        //return guess.parse::<u8>().expect("You didn't enter an integer");

        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(_e) => println!("You didn't enter an integer")

        };
    }
}

fn handle_guess (guess : u8, correct_num : u8) -> bool {
    if guess < correct_num {
        println!("Your guess was lower than the correct answer");
        false
    } else if guess > correct_num {
        println!("Your guess was higher than the correct answer");
        false
    } else {
        println!("You got it!");
        true
    }
}

fn main() {
    let random_num: u8 = random::<u8>();

    println!("Hello, welcome to the guessing game!");

    loop {
        let guessed_num: u8 = get_guess();

        let guess_result: bool = handle_guess(guessed_num, random_num);

        if guess_result {
            break
        }
    }



}
