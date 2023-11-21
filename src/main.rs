fn main() {
    // cause panic
    // let a = vec![1, 2, 3];
    // a[99];

    // handle different types of errors
    // use std::fs::File;
    // use std::io::ErrorKind;

    // let greeting_file_result = File::open("hello.txt");
    // match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     }
    // };

    // use closure simplify the code
    // use std::fs::File;
    // use std::io::ErrorKind;

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });

    // use unwrap and expect
    // use std::fs::File;

    // let file = File::open("hello.txt").unwrap();
    // let file = File::open("hello.txt").expect("failed to open file");

    // use ? operator
}

// use std::fs::File;
// use std::io::{Error, Read};

// fn read_username_from_file() -> Result<String, Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }
