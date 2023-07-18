//! Copyright (c) Liam 2022
//! All rights reserved. Licensed under MIT.
//!
//! The examples about recoverable errors in Chapter 9.2

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn match_result() {
    let greating_file_result = File::open("hello_match.txt");

    let greating_file = match greating_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello_match.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn unwrap_result() {
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, call the closure to handle
    let greeting_file = File::open("hello_upwrap_or_else.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_upwrap_or_else.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // If the Result value is the Ok variant, unwrap will return the value inside the Ok.
    // If the Result is the Err variant, unwrap will call the panic! macro
    let greeting_file = File::open("hello_upwrap.txt").unwrap();
}

fn expect_result() {
    let greeting_file = File::open("hello_expect.txt")
        .expect("hello_expect.txt should be included in this project");
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    // redundant coding style for propagating errors
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    // consict coding style for propagating errors
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn main() {
    match_result();
    unwrap_result();
    expect_result();
    read_username_from_file_1();
    read_username_from_file_2();
    read_username_from_file_3();

}
