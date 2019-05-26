use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let to_be_guessed = rand::thread_rng().gen_range(0, 77);
    
    // println!("The number to be guessed is {}", to_be_guessed);

    println!("Let's play a guessing game!");

    println!("Please input your guess");

    let mut guess =  String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line :(");


    let guess:u32 = guess.trim().parse()
        .expect("Please type a number!");


    match guess.cmp(&to_be_guessed) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("You guessed: {}", guess);

}
