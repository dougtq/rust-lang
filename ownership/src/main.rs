fn main() {
    let mut s = String::from("hello"); // // s is valid from this point forward

    s.push_str(", world!"); // // Adciona texto no fim da string

    println!("{}", s);


    let s1 = String::from("hello");
    let s2 = s1;

     println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {} and s2 = {}", s1, s2);


    let txt = String::from("hello");  // txt comes into scope

    takes_ownership(txt);             // txt's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still


    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);


    let mut s = String::from("hello");

    {
        let r1 = &mut s;

    } // Maneira funcional de utilizar duas referencias mutaveis da mesma variavel

    let r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 e r2 fora de escopo, ja q não são mais usadas

    let r3 = &mut s; // no problem
    println!("{}", r3);
}


fn calculate_length(s : &String) -> (usize) {
    let length = s.len();

    length
}

// // this scope is now over, and s is no longer valid

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
}

  // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
