#[derive(Copy, Clone)]
enum MachineState {
    Normal,
    Comment,
    Lowercase,
    Uppercase,
}

enum Option {
    Some(char),
    None,
}

fn main() {
    let mut state = MachineState::Normal;
    let mut processed_str = String::new();

    let txt = "This _IS_ ^some input^#ignore what's written here pls#, nice markup language!!!";

    for character in txt.chars() {
        let (output, new_state) = machine_cycle(character, state);

        state = new_state;

        if let Option::Some(c) = output {
            processed_str.push(c);
        }
    }

    println!("{}", processed_str);
}

fn machine_cycle(c: char, state: MachineState) -> (Option, MachineState) {
    match (state, c) {
        (MachineState::Normal, '#') => (Option::None, MachineState::Comment),
        (MachineState::Normal, '^') => (Option::None, MachineState::Uppercase),
        (MachineState::Normal, '_') => (Option::None, MachineState::Lowercase),
        (MachineState::Normal, other) => (Option::Some(other), MachineState::Normal),
        (MachineState::Comment, '#') => (Option::None, MachineState::Normal),
        (MachineState::Comment, _other) => (Option::None, MachineState::Comment),
        (MachineState::Uppercase, '^') => (Option::None, MachineState::Normal),
        (MachineState::Uppercase, other) => (Option::Some(other.to_ascii_uppercase()), MachineState::Uppercase),
        (MachineState::Lowercase, '_') => (Option::None, MachineState::Normal),
        (MachineState::Lowercase, other) => (Option::Some(other.to_ascii_lowercase()), MachineState::Lowercase),
    }
}
