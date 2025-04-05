//This file contains the basic Rust functions and operations

fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn run() {

    println!("My name is Ziyad.");

    let year = 2025;
    println!("The year is: {}", year);


    let result = add(10, 20);
    println!("The sum of 10 and 20 is: {}", result);

}