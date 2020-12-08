fn main() {
    let a = 3;

    let b = &a;

    do_print(b);
}

fn do_print(value : &i32) {
    println!("Here is the value {}", value);
}
