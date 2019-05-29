fn main() {

    // Integer
    let _guess: u32 = "42".parse().expect("Not a number!");

    println!("Valor: {}", _guess);

    // Float
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    /* Math */
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;


    // Bool
    let t = true;

    let f: bool = false;


    // Char
    let c = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Valor char: {}", c);


    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    println!("The value of position 1 is: {}", tup.1);

    // Array (Must be same type and fixed length)
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
