#[macro_use]
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input the miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);
    println!("Input the difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("It should be an integer");
    
    println!("Generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), diff);

    loop {
        println!("Menu: ");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        println!("Enter your choice: ");

        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
    
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting...");
                process::exit(0);
            },
            1 => {
                println!("New Transaction");
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                println!("Enter the receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);


                println!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let result = chain.new_transaction(sender, receiver, amount.trim().parse().unwrap());

                match result {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed")
                }
            },
            2 => {
                println!("Genearting new block");
                let result = chain.generate_new_block();

                match result {
                    true => println!("Block generated successfully"),
                    false => println!("Could not generate block")
                }
            },
            3 => {
                println!("Update Difficulty");
                let mut new_difficulty = String::new();


                println!("Enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_difficulty);

                let result = chain.update_difficulty(new_difficulty.trim().parse().unwrap());

                match result {
                    true => println!("Difficulty updated"),
                    false => println!("Failed to updated Difficulty")
                }
            },
            4 => {
                println!("Update Rewatd");
                let mut new_reward = String::new();


                println!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);

                let result = chain.update_reward(new_reward.trim().parse().unwrap());

                match result {
                    true => println!("Reward updated"),
                    false => println!("Failed to update reward")
                }
            },
            _ => println!("\t Invalid option please retry \t"),
        }
    }
}
