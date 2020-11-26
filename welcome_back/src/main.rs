mod teste;

use teste::variavel::var;
use teste::temp as temporario;

fn main() {
    println!("Hello, world!");
    temporario();
    var();
    var_test();
}


fn var_test() {
    let mut a = 1;
    let vetor = vec![1, 3, 4];

    a += 10;

    println!("{}", a);
    println!("{}", vetor[1]);
}
