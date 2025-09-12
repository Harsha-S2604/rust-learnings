#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle1: Rectangle = Rectangle {
        width: dbg!(12 * 4),
        height: 23,
    };

    let rect_area: u32 = get_rect_area(&rectangle1);
    
    println!("The area of rectangle is {rect_area}");
    dbg!(&rectangle1);
}

fn get_rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area(length: u32, breadth: u32) -> u32 {
    length * breadth
}
