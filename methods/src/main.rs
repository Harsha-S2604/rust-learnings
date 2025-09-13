#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn from(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height 
    }
}

fn main() {
    let rectangle1 = Rectangle {
        width: 12,
        height: 23,
    };

    let rectangle2 = Rectangle {
        width: 11,
        height: 20,
    };
    
    let rectangle3 = Rectangle::from(12, 34);

    println!("The width and height of rectangle 3 is {:?} and {:?}", rectangle3.width, rectangle3.height);
}
