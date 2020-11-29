use std::collections::LinkedList;
use std::env;

fn main() {
    let i1 : i32 = 53;
    let i2 : i64 = 27;


    println!("The values added equals to {}", add(i1, i2));
    println!("-----------");

    let mut ll = LinkedList::new();

    ll.push_back(1);
    ll.push_back(5);
    ll.push_back(6);

    for item in ll {
        println!("{}", item)
    }

    println!("-----------");

    let mut v = Vec::new();

    v.push('x');
    v.push('z');
    v.push('u');

    for item in v {
        println!("{}", item)
    }

    println!("-----------");

    convo::say_hello();

    println!("-----------");

    let vars = env::vars();

    for (key, val) in vars {
        println!("{} value is {}", key, val)
    }
}


fn add (a: i32, b: i64) -> i64 {
    return (a as i64) + b;
}

mod convo {
    pub fn say_hello () {
        println!("Hello, world!")
    }

}
