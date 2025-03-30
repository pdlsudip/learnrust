use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
pub fn errorfn() {
    let result_r = File::open("hello.txt");
    let _final_result = match result_r {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Error creating {}", error),
            },
            _ => panic!("Error"),
        },
    };
    let write_buffer = String::from("Hello from rust");
    fs::write("hello.bin", write_buffer).unwrap();
    let mut buffer = String::new();
    let mut file = File::open("hello.bin").unwrap();
    match file.read_to_string(&mut buffer) {
        Ok(_) => println!("{:?}", buffer),
        Err(_) => panic!(),
    };
}
