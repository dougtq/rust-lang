fn main() {
    let mut it = vec!["A", "F", "D"].into_iter();

    let mut enum_it = it.enumerate();

    println!("Enum {:?}", enum_it.next());


    let mut num_it = vec![1 .. 999999].into_iter();

    num_it.skip(200);

    let mut taken = num_it.take(50);
    let coll : Vec<i32> = taken.collect();

    println!("Enum Taken {:?}", coll);
}
