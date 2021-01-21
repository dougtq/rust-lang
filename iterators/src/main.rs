fn main() {
    let mut it = vec!["A", "F", "D"].into_iter();

    let mut enum_it = it.enumerate();

    println!("Enum {:?}", enum_it.next());


    let num_it = (1..10).into_iter();

    let other : Vec<i32> = num_it.map(|x| {
        x as i32 + 3
    }).collect();

    println!("Map {:?}", other);
}
