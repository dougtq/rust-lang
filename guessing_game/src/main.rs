use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let to_be_guessed = rand::thread_rng().gen_range(0, 77);
    
    // println!("The number to be guessed is {}", to_be_guessed);

    println!("Adivinhe o numero!");
    loop {
        println!("Digite um numero e pressione Enter!");

        let mut guess =  String::new();

        io::stdin().read_line(&mut guess)
            .expect("Falha ao ler o que você digitou :(");


        let guess:u32 = guess.trim().parse()
            .expect("Não foi digitado um numero");

        match guess.cmp(&to_be_guessed) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você venceu :):):):):) !");
                break;
            }
        }

        println!("You guessed: {}", guess);
    }

}
