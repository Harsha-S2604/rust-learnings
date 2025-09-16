#[derive(Debug)]
enum InputBox {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // there are two ways to declare the vectors
    let mut nums: Vec<i32> = Vec::new();
    nums.push(100);
    nums.push(250);
    nums.push(310);
    
    for (index, num) in nums.iter().enumerate() {
        println!("index - Num: {index} {num}");
    }

    let third_num: Option<&i32> = nums.get(5);

    if let Some(third_num) = third_num {
        println!("Third num is {third_num}");
    } else {
        println!("None");
    }

    let mut freq = vec![1, 2, 3, 4 ,5];
    let first_freq = &freq[0];
    freq.push(120);

    for num in &freq {
        println!("{num}");
    }

    println!("{:?}", freq);

    let mut input_box_1 = vec![
        InputBox::Int(12),
        InputBox::Text(String::from("Hello")),
        InputBox::Float(12.23),
    ];

    for value in &input_box_1 {
        match value {
            InputBox::Int(number) => {
                println!("We got a Number");
            },

            InputBox::Text(text) => {
                println!("we got a text");
            },

            InputBox::Float(float) => {
                println!("We got a float");
            },
        }
    }
}
