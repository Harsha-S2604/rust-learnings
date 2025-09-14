#[derive(Debug)]
enum JSON {
    JsonBoolean(bool),
    JsonString(String),
    JsonNull,
    JsonNumber(isize),
}

fn get_string_value(str_enum: JSON) -> Option<String> {
    let JSON::JsonString(str_value) = str_enum else {
        return None;
    };

    Some(str_value)
}

fn main() {
   let json_bool_value = JSON::JsonBoolean(false);
   let json_string = JSON::JsonString(String::from("name"));
    
   if let JSON::JsonBoolean(value) = json_bool_value {
       println!("The boolean value is {:?}", value);
   }

    let str_some_value = get_string_value(json_string);
    
    if let Some(ref str_value_1) = str_some_value {
        println!("if let -- The string value is {str_value_1}");
    } else {
        println!("if let -- The string value is None"); 
    }

    println!("The value is {:?}", str_some_value);

}
