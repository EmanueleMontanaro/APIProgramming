use std::fs::OpenOptions;
use std::io::prelude::*;


fn readFile(filename: String) {
    match OpenOptions::new().read(true).write(true).open(filename) {
        Ok(mut file) => {
            let mut contents;
            if let Err(err) = file.read(&mut contents) {
                eprintln!("Error reading file: {}", err);
            } else {
                println!("File contents:\n{}", contents);
                for i in 0..10 {
                    if let Err(err) = file.write_all(contents.as_bytes()) {
                        eprintln!("Error writing file: {}", err);
                    } else {
                        println!("File written");
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
}

fn main() {
    readFile(String::from("src/test.txt"));
}
