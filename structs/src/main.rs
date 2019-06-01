use std::fmt;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // add code here
    fn area (&self) -> u32 {
        self.height * self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {}, {}, {})", self.username, self.email, self.sign_in_count, self.active)
        }
    }

    let mut user1 = User {
        username: String::from("doug123"),
        email: String::from("someone@example.com"),
        sign_in_count: 0,
        active: false
    };

    user1.email = String::from("anotheremail@example.com");

    println!("Usuario criado: {}", user1);

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("Retangulo  desejado: {:#?} ", rect1);

    println!("A area do retangulo é de: {} pixels.", rect1.area());

    println!("{:#?}", Rectangle::square(45));

}
