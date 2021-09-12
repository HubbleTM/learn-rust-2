use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    // panic!("Hello, world!");
    let text = open_file_refactored_2();
    match text {
        Ok(str) => println!("{}", str),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found : {:?}", error),
            _ => println!("Got an error : {:?}", error),
        }
    };
}


fn open_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut file = match f {
        Ok(file) => {
            file
        }
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn open_file_refactored() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

fn open_file_refactored_2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}