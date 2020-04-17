use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    println!("Hello from a main thread!");
    thread::sleep(Duration::from_millis(20));
}

/*
fn main() {
    println!("Hello, world!");
    println!("HOGEHOGEHOGE");
}
*/

