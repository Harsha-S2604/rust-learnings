fn get_median(arr: &[i32]) -> i32 {
    let start = 0;
    let end = arr.len() - 1;

    let mid_idx: usize = start + (end - start) / 2;
    
    let median = arr[mid_idx];

    median
}

fn str_to_pig_latin(input: &mut String) -> String {
    let mut pig_latin = String::from("");

    for word in input.split_whitespace() {
        let first_char = &word[..1];
        let other_part = &word[1..];
        let s = format!("{}{}ay ", other_part, first_char);
        pig_latin.push_str(&s);
    }
    pig_latin.trim_end().to_string()
}

fn main() {
    let mut input = String::from("sally knows the best");
    let pig_latin = str_to_pig_latin(&mut input);

    println!("{:#?}", pig_latin);
}
