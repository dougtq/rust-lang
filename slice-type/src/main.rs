fn main() {
    let mut text = String::from("Hello darkness");

    let first_word_index = first_word(&text);
    
    let first = &text[..first_word_index];
    let second = &text[first_word_index +1 ..];

    println!("Index of end of first word: {}", first_word_index);
    println!("First word: {}", first);
    println!("Second word: {}", second);


    text.clear();

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

fn first_word(text: &str) -> usize {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    text.len()
}
