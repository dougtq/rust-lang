fn main() {

    /* Variaveis por padrão são imutaveis, mut define como mutavel */
    let mut x = 5;  
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /* Shadowing de variaveis  */
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of x is: {}", y);

    /* Shadowing de variaveis com mudança de tipo */
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The length is: {}", spaces);
}
