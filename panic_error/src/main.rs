#![allow(unused)]
use core::panic;
use std::{fs::File, io::ErrorKind};
use std::{fs, io};
use std::io::Read;

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];

    // v[99];
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file: {:?}", e),
            }
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!{"Problem opening the file: {:?}", error};
        }
    });

    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open");

    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// fn read_username_from_file2() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)

//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)

//     fs::read_to_string("hello.txt");
// }