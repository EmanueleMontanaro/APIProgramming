
// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use rand::random;

fn main() {
    let x:i32 = rand::random();
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
