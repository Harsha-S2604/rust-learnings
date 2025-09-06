fn main() {
    // DATA TYPES
    println!("MAN OF MASSES:: DATA TYPES!");

    // two data types
    // 1. scalar 2. compound
    // scalar types are:
    // 1. Integer 2. Float 3. Booleans 4. Characters

    // Integer
    // types: 8,16,32,64,128, architecture dependent
    // all types has signed and unsigned type
    /* How the number stores in the system
     * Signed numbers are stored using two's complement
     *
     * Two's complement represent the signed integers
     *
     * It has 3 main props:
     *  1. Positive Number
     *  2. Negative Number
     *  3. Zero
     *
     * How it works
     *  Positive number -> Stored as normal binary
     *      For example: (8 bit number)
     *          5 -> 0000 0101
     *          0 -> 0000 0000
     *  Negative Number -> stored using 2's complement
     *      Rule: TO get the two's complement of a number:
     *          1. write it's binary form
     *          2. invert all the bits(0 -> 1, 1 -> 0)
     *          3. Add 1
     *      Example: (8-bit storage)
     *          Positive 5
     *              0000 0101 (same as unsigned)
     *          Negative 5
     *              5 -> 0000 0101
     *              1's complement -> 1111 1010
     *              Add 1 -> 1111 1011 which is equal to -5
     * You might be having question isn't 1111 1011 is equal to 251
     *
     * Yes it's true 1111 1011 equals to 251 only if the data type is u8
     * if u declare i8 = -5 then it is also equal to 1111 1011
     *
     * now, What if i32 = 251? which is signed 32-bit integer
     * then it will store as
     * 00000000 00000000 00000000 11111011 -> here it represents
     * the positive integer So no worries
     *
     * now, what if i8 = 127? which is signed 8-bit integer and it consists
     * of 1 as the MSB which represents negative but it is positive.
     * Here 1 bit is left for positive/negative sign
     *
     * 127 -> 01111111 here 127 represents as the positive number
     * 
     * overflow - in release version, Rust performs two's complement
     * wrapping
     *
     * */
    
    println!("===== SCALAR DATA TYPES =====");
    // INTEGER
    // 8-bit
    let s_8_bit: i8 = 12;
    let us_8_bit: u8 = 127;
    
    println!("8 bit signed integer value {s_8_bit}");
    println!("8 bit unsigned integer value {us_8_bit}");

    // 16-bit
    let s_16_bit: i16 = 12;
    let us_16_bit: u16 = 1200;

    println!("16 bit signed integer value {s_16_bit}");
    println!("16 bit unsigned integer value {us_16_bit}");


    //64-bit
    let s_64_bit: i64 = -12340;
    let us_64_bit: u64 = 123456789;

    println!("64 bit signed integer value {s_64_bit}");
    println!("64 bit unsigned integer value {us_64_bit}");
    // 128-bit
    let s_128_bit: i128 = -12345678;
    let us_128_bit: u128 = 987654321;

    println!("128 bit signed integer value {s_128_bit}");
    println!("128 bit unsigned integer value {us_128_bit}");
    // architecture dependent
    let s_size: isize = 12345;
    let us_size: usize = 123456;

    println!("arch size signed integer value {s_size}");
    println!("arch size unsigned integer value {us_size}");

    // Floating point types
    // It has two bits in size
    // 1. 32 bits and 64 bits
    // default type is f64 because of more precision
    // all are signed float point

    let float_num = 12.0; // default to f64
    let float_num_32 = 2.123;

    println!("The value of 64 bit float num is {float_num}");
    println!("The value of 32 bit float num is {float_num_32}");

    // Boolean
    // true and false
    // One byte in size(8-bit)
    let t = true;
    let f: bool = false;

    println!("The value of t is {t}");
    println!("The value of f is {f}");

    // Character
    // four byte
    let c: char = 'z';
    let z: char = 'c';

    println!("The character is {c}");
    println!("The character is {z}");

    // COMPOUND DATA TYPES
    println!("\n===== COMPOUND DATA TYPES =====");

    // Compound data types can group multiple values into one type
    // It has two types
    // 1. Tuple 2. Array

    // TUPLE
    // Once declared cannot increase or shrink the size
    let tup: (i32, i64, char) = (500, 12, 'c');
    let (x, y, z) = tup;

    println!("The first value of tuple is {x}");
    println!("The second value of tuple is {y}");
    println!("The third value of tuple is {z}");

    println!("Accessing using tup using dot: {0} {1} {2}", tup.0, tup.1, tup.2);
   

    // Array
    // must have same type and has a fixed length
    let a = [1, 2, 3, 4, 5]; // array of i32 
    let months = ["jan", "feb", "mar", "June", "July", "Aug"]; // array of strings
    let arr: [i32; 5] = [10, 20, 30, 40, 50];
    println!("The array is {:?}", arr);

    let bool_arr = [true; 4]; // sets 4 values to true
    println!("Bool array: {:?}", bool_arr);


    println!();
}

