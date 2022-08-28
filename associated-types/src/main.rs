trait X {}

trait Y {}

trait Z: X + Y {}

trait ShowMyself {
    fn show(&self) -> String;
}

struct A;

impl X for A {}
impl Y for A {}
impl Z for A {}

impl ShowMyself for i32 {
    fn show(&self) -> String {
        format!("I'm a i32: {}", *self)
    }
}

impl ShowMyself for String {
    fn show(&self) -> String {
        format!("I'm a String: {}", *self)
    }
}

fn show<T: ShowMyself>(x: T) {
    println!("{}", x.show());
}

fn main() {
    println!("Hello, world!");
    let txt: String = "Hello, world!".to_string();
    let num: i32 = 55;

    show(txt);
    show(num);
}
