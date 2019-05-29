fn main() {
    alternative(2);

    let x = 5;

    let y = {
        let x = 3;
        x + 1 /* Linha sem ponto e virgula define como resultado de retorno*/
    };

    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    println!("The value of five fn is: {}", five());
    println!("The value of plus_one fn is: {}", plus_one(five()));
}

fn alternative(num: i32) {
    println!("O valor é: {}", num);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}