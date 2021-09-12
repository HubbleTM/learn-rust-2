use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    // panic!("Hello, world!");
    let text = open_file();
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
        },
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}