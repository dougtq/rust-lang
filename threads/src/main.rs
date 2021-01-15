use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Running on other thread");
    });

    println!("Running before a thread");

    handle.join();
}
