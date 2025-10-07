use std::thread;
use std::sync::{Arc, Mutex};
use std::rc::Rc;
fn main() {
    let m = Mutex::new(6);

    {
        // this returns the MutexGuard which implements the Deref
        let mut num = m.lock().unwrap();
        *num = 12;
    }

    println!("M is: {m:#?}");

    // let counter = Rc::new(Mutex::new(0)); # not thread safe. good for single threaded application

    let counter = Arc::new(Mutex::new(10)); 
    let mut handlers = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("The value of counter is {:#?}", *counter.lock().unwrap());
}
