#![allow(unused)]
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct MyStruct {
    name: String,
}

impl std::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Error for MyStruct {}

pub struct NoErrorTrait;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("data.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    let _ = read_file();
}
