trait Vehicle {
    fn new(name: &'static str) -> Self;
    fn movement(&self) -> ();
    fn to_string(&self) {
        println!("Vehicle Name:")
    }
}

trait NoisyAnimal {
    fn make_noise(&self) -> &'static str;    
}

struct Airplane { name: &'static str }
struct Bicycle { name: &'static str }
struct Car { name: &'static str }



struct Cat {}
struct Dog {}

impl Vehicle for Airplane {
    fn new(name: &'static str) -> Self {
        Airplane { name: name }
    }
    fn movement(&self) {
        println!("moving")
    }
}

impl NoisyAnimal for Cat {
    fn make_noise(&self) -> &'static str {
        "meoooooow"
    }
}

impl NoisyAnimal for Dog {
    fn make_noise(&self) -> &'static str {
        "wwwwwwwwwooooooooooooooooooooooooooooooooof"
    }
}


fn make_noises(animals: Vec<Box<dyn NoisyAnimal>>) {
    for animal in animals {
        println!("{}", animal.make_noise());
    }
}


fn main() {
    println!("Hello, world!");

    let animals: Vec<Box<dyn NoisyAnimal>> = vec![
        Box::new(Cat{}),
        Box::new(Dog{}),
        Box::new(Cat{}),
        Box::new(Dog{}),
    ];

    make_noises(animals);
}


fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy {
    let mut largest = list[0]; // Copy trait

    for &item in list.iter() { //Copy trait
        if item > largest {
            largest = item;
        }
    }

    largest
}