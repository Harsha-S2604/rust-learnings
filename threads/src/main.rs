use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    let thread_1_handler = thread::spawn(|| {
        println!("Running thread 1...");
        for i in 1..10 {
            println!("Hello from thread 1 {i}");
            thread::sleep(Duration::from_secs(1));
        }
    });

    let thread_2_handler = thread::spawn(|| {
        println!("Running thread 2...");
        for i in 1..10 {
            println!("Hello from thread 2 {i}");
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread_1_handler.join().unwrap();
    thread_2_handler.join().unwrap();
}
