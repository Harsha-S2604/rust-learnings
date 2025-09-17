use std::fs::File;
use std::io;

fn open_file(file_name: &str) -> Result<File, io::Error> {
    let file = File::open("abc.txt")?;
    Ok(file)
}

fn main() {
    // let file = open_file("abc.txt");

    /* match file {
        Ok(file) => println!("found a file"),
        Err(e) => println!("Sorry file not found"),
    }*/
    
    // will return Ok if the file present or panics
    // if it's error
    // let file = File::open("abc.txt").unwrap();
    
    // same as above but we can give our own message for error
    let file = File::open("abc.txt").expect("File not present");
}
