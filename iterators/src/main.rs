fn main() {
    let it = vec!["A", "F", "D"].into_iter();

    let mut enum_it = it.enumerate();

    println!("Enum {:?}", enum_it.next());

    let map_it = (1..10).into_iter();
    let filter_it = (1..10).into_iter();
    let for_it = (1..10).into_iter();
    let fold_it = (1..10).into_iter();

    let other : Vec<i32> = map_it.map(|x| {
        x as i32 + 3
    }).collect();

    println!("Map {:?}", other);

    let other_filtered : Vec<_> = filter_it.filter(|x| { x % 2 == 0 }).collect();

    println!("Filter {:?}", other_filtered);

    println!("Fold {:?}", fold_it.fold(0, |sum, item| { sum + item }));

    let mut other_items : Vec<i32> = Vec::new();

    for item in for_it {
        other_items.push(item + 3)
    }
    println!("for..in {:?}", other_items);

}
