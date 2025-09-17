#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest_num = &list[0];

    for num in list {
        if num > largest_num {
            largest_num = num;
        }
    }

    largest_num
}

fn largest_char(list: &[char]) -> &char {
    let mut largest_char = &list[0];

    for character in list {
        if character > largest_char {
            largest_char = character;
        }
    }

    largest_char
}

//fn largest_generic<T>(list: &[T]) -> &T {
//    let mut largest_val = &list[0];
//
//    for value in list {
//        if value > largest_val {
//            largest_val = value;
//        }
//    }
//
//    largest_val
//}

fn main() {
    let list = vec![1, 2, 3, 4, 5];
    let char_list = vec!['a', 'b', 'c', 'd'];
    
    let largest_num = largest(&list);
    let largest_char = largest_char(&char_list);
    
    let point1 = Point { x: 5, y: 10 };
    let point2 = Point { x: 12.2, y: -10.0 };
    let point3 = Point2 { x: 12, y: 12.2 }; 
    println!("point 1 -> {:#?}", point1.x());
    println!("Point 3 -> {:#?}", point3);
    println!("The largest number is {largest_num}");
    println!("The largest character is {largest_char}");
}
