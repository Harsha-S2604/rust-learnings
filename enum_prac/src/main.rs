#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

#[derive(Debug)]
enum IpAddrKindType {
    v4(String),
    v6(String),
}

#[derive(Debug)]
enum IpAddrKindTuple {
    v4(u8, u8, u8, u8),
    v6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
    Move {x: i32, y: i32},
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
       println!("{:#?}", self); 
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let ip_address = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("192.0.0.1"),
    };
    
    let home = String::from("127.0.0.1");
    let ip_address_type = IpAddrKindType::v4(home);
    
    let ip_addr_tuple = IpAddrKindTuple::v4(127, 0, 0, 1);
    println!("The type of the address is {:?}", ip_addr_tuple);
    

    let m = Message::Write(String::from("Hello!"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    println!("The value of y is {y}");
}
