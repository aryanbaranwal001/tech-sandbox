use anyhow::Result;
use std::fs;

fn read_number() -> Result<i32> {
    let s = fs::read_to_string("num.txt")?;
    let n: i32 = s.trim().parse()?;
    Ok(n)
}

fn main() -> Result<()> {
    let n = read_number()?;
    println!("number: {}", n);
    Ok(())
}
