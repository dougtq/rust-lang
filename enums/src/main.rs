#[derive(PartialEq)]
enum Animal {
    Dog,
    Cat,
    Rabbit,
}

enum Relationship {
    Father,
    Mother,
    Son,
    Daughter,
    Other(u8),
}

enum Action {
    Drive,
    Turn(Direction),
    Grab,
    Stop,
}

enum Direction {
    Left,
    Right,
}

fn print_action(a: Action) {
    match a {
        Action::Drive => println!("Drive forward"),
        Action::Turn(direction) => match direction {
            Direction::Right => println!("Turn Right"),
            Direction::Left => println!("Turn Left")
        },
        Action::Stop => println!("Stops moving"),
        Action::Grab => println!("Grabs object")
    }
}

fn main() {
    let my_pet = Animal::Rabbit;
    let my_other_pet = Animal::Cat;

    let program = vec![
        Action::Drive,
        Action::Turn(Direction::Right),
        Action::Drive,
        Action::Grab,
        Action::Turn(Direction::Right),
        Action::Turn(Direction::Right),
        Action::Drive,
        Action::Turn(Direction::Left),
        Action::Drive,
        Action::Stop,
    ];

    for action in program {
        print_action(action);
    }

    //assert!(my_pet == my_other_pet);
}
