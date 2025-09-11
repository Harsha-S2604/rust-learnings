struct Point(i32, i32);

struct User {
    name: String,
    active: bool,
    email: String,
}

fn get_user_instance(name: &str, email: &str) -> User {
    let user = User {
        name: String::from(name),
        email: String::from(email),
        active: true,
    };
    user
}

fn main() {
    let mut user: User = get_user_instance("john doe", "john@gmail.com");
    let mut user2: User = get_user_instance("Jack", "jack@gmail.com");
    
    let mut user3: User = User {
        name: String::from("Joe"),
        email: String::from("joe@gmail.com"),
        active: true,
    };
    
    let mut point: Point = Point(-12, 12);
    println!("The email of user3 is {0}", user3.email);

    /* struct by default should contain owned values
     if any case u want to do reference then use lifetime option
     for example:
        struct User {
            name: &str,
        }
        
        let name = String::from("john");
        let user = User {
            name: &name
        }

        the above code will throw error, as the reference is not accepted,
        we can use life time to avoid such errors, we can discuss about
        lifetime in another module
    */        
}
