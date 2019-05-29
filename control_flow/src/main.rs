fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    /* 
        Um if com apenas um valor, sendo ele diferente de bool gera um erro 
        if number {  ERROR
            println!("number was something other than zero");
        }
    
    */

    if number != 0 {
        println!("number was something other than zero");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The NEW value of number is: {}", number);


    // Loops
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..6).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
