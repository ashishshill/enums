// Error handeling
// Panic example for rust program.
fn main() {
    a();
}

fn a() {
    b();
}

// if we update the value c then it will be okay
fn b() {
    // update 22 to 21 for error
    c(21);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Dont pass is 22!");
    }
}

use std::error;
use std::fs::File;
use std::io::ErrorKind;
// error handeling for resulat enum
// fn main() {
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
// }
// another error handling
fn main() {
    let f = File::open("Hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file{:?}", e),
            },
            other_error => {
                panic!("Problem opeing the file:{:?}", other_error)
            }
        },
    };
    // More good pactice for error handling
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file:{:?}", error);
        }
    });
}
