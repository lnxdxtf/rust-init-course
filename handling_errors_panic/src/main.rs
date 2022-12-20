#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use std::fs::{File, rename};
use std::io::{ErrorKind, Error};

fn error_handling_try_catch() {
    let file = File::open("err.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("err.txt") {
                Ok(file_created) => file_created,
                Err(err) => panic!("Cannot create file - err: {:?}", err)
            },
            _ => panic!("occurred error"),
        }
    };
}


fn working_with_unwrap() {
    // let file_test = File::open("test.txt").unwrap();
    let file_test_2 = File::open("test.txt").expect("Error opening test file");
}

fn error_propagation() {
    fn open_file() -> Result<File, Error>{
        let _file = File::open("err.txt")?;
        Ok(_file)
    }
    fn rename_file() -> Result<(), Error> {
        let _file = rename("err.txt", "renamed.txt")?;
        Ok(_file)
    }
    let _test = open_file();
    _test.unwrap();
    let _secondTest = rename_file();
    _secondTest.unwrap();
}

fn main() {
    // error_handling_try_catch();
    // working_with_unwrap();
    error_propagation();
}
