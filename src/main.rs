use std::fs::File;
use std::io::{prelude::*};

fn main() -> std::io::Result<()> {
    let mut file = File::open("C:/Programming/advent_of_code_rust/input/day1a.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut count: i32 = 0;
    let mut last_value: i32 = 0;
    let mut former_value: i32 = 0;
    let mut current_value: i32 = 0;
    let mut next_value: i32 = 0;
    let mut former_sum: i32 = 0;
    let mut current_sum: i32 = 0;

    let mut loop_counter: i32 = 0;

    for s in contents.lines() {
        next_value = s.parse::<i32>().unwrap();

        if 2 < loop_counter {
            former_sum = last_value + former_value + current_value;
            current_sum = former_value + current_value + next_value;

            if former_sum < current_sum {
                count += 1;
            } 
        }
        last_value = former_value;
        former_value = current_value;
        current_value = next_value;
        loop_counter += 1;
    }

    println!("Successfully found {} positive sum deltas", count);
    
    
    
    Ok(())
}
