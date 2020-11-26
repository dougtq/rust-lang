use rand::prelude::random;

pub fn var() {
    let a = random::<i8>();
    println!("var fn! {}", a)
}
