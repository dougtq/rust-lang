use std::collections::HashMap;

const FIB_ZERO : u64 = 1;
const FIB_ONE : u64 = 1;

fn main() {
    println!("Hello, world!");

    let mut map = HashMap::new();

    for i in 1..41 {
        println!("{} : {}", i, fib_dyn(i, &mut map))
    }
}

fn fib(num : u64) -> u64 {
    if num == 0 {
        FIB_ZERO
    } else if num == 1 {
        FIB_ONE
    } else {
        fib(num - 1) + fib(num - 2)
    }
}


fn fib_dyn (num : u64, map: &mut HashMap<u64, u64>) -> u64 {
    match num {
        0 | 1 => 1,
        num => {
            if map.contains_key(&num) {
                *map.get(&num).unwrap()
            } else {
                let val = fib_dyn(num - 1, map) + fib_dyn(num - 2, map);
                map.insert(num, val);

                return val
            }
        }
    }
}

