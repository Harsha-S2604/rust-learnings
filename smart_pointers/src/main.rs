use std::ops::Deref;
// Custom smart pointer
struct MyBox<T>(T);
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer {} !", self.data);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    //let x = 5; // stores on the stack
    // let y = Box::new(x); // copies from x and stores the data on heap
    
    let x = 5;
    let y = MyBox::new(x);
    println!("y = {0}", *y);

    let c = CustomSmartPointer {
        data: String::from("My Stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("Another Stuff"),
    };

    let e = CustomSmartPointer {
        data: String::from("E Custom Stuff"),
    };

    let box_instance = Box::new(String::from("Hello"));

    drop(box_instance);
}
