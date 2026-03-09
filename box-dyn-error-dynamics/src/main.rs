#![allow(unused)]
use std::error::Error;

#[derive(Debug)]
struct MyErrorA;
#[derive(Debug)]
struct MyErrorB;

impl std::fmt::Display for MyErrorA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyErrorA")
    }
}

impl std::fmt::Display for MyErrorB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyErrorA")
    }
}

impl Error for MyErrorA {}
impl Error for MyErrorB {}

fn example() -> Result<i32, Box<dyn Error>> {
    // Err(Box::new("hello".to_string()))
    Err(Box::new(MyErrorA))
}

fn main() {
    let _ = example();
}
