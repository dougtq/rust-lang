fn main() {
    let a = vec![555, 3 , 278, 774];
    let sla = &a[1..3];


    let b = String::from("Hello World!");
    let slb = &b[6..11];

    println!("Slices: {:?} - {:?}", sla, slb);
}
