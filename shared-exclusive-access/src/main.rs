fn main() {
    let values = vec![1, 5, 8, 20];
    let mut result : i32 = take_ownership_sum(values);
    println!("The total sum result in {}", result);

    let vector = vec![1, 5, 8, 20];
    result = borrow_sum(&vector);
    println!("The total sum of {:?} ints result in {}", vector, result);

    let mut vec_values = vec![57, 325, 800, 4700];
    let max_value = 100;
    cap_values_borrow(&max_value, &mut vec_values);
    println!("{:?}", vec_values);

}


fn take_ownership_sum (v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += value;
    }

    return sum;
}

fn borrow_sum (v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += *value;
    }

    return sum;
}

fn cap_values_borrow (max :&i32, v: &mut Vec<i32>) {
    for index in 0..v.len() {
        if v[index] > *max {
            v[index] = *max
        }
    }
}
