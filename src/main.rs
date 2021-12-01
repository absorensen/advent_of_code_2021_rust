use std::fs::File;
use std::io::{prelude::*};

fn main() -> std::io::Result<()> {
    let mut file = File::open("C:/Programming/advent_of_code_rust/input/day1a.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut count: i32 = 0;
    let mut former_value: i32 = 0;
    let mut current_value: i32 = 0;
    let mut first_loop: bool = true;

    for s in contents.lines() {
        current_value = s.parse::<i32>().unwrap();

        if first_loop {
            first_loop = false;
        }
        else if former_value < current_value {
            count += 1;
        } 
    
        former_value = current_value;

    }

    println!("Successfully found {} positive deltas", count);
    
    
    
    Ok(())
}
