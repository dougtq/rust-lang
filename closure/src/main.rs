fn main() {
    let val = 26;
    let clo = |x| { x + val};

    println!("The result is {}", clo(1));
}
