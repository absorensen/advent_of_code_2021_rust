use std::fs::File;
use std::io::{prelude::*};

fn parse_txt_file_to_int_vec(path: &str) -> Vec<i32>{
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut result : Vec<i32> = Vec::<i32>::new();
    for s in contents.lines() {
        result.push(s.parse::<i32>().unwrap());
    }

    result
}

fn b_01_21(use_functional: bool) -> usize {
    let signal = parse_txt_file_to_int_vec("C:/Programming/advent_of_code_rust/input/day1a.txt");
    
    if use_functional {
        let count = signal
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();
    
        println!("b_01_21: Successfully found {} positive sum deltas", count);
    
        count
    } else {
        let mut moving_window = [signal[0], signal[1], signal[2], signal[3]];
        let mut count: usize = 0;
        for element in 3..signal.len(){
            moving_window[3] = signal[element];
            if moving_window[0] + moving_window[1] + moving_window[2] < moving_window[1] + moving_window[2] + moving_window[3]{
                count += 1;
            }
            moving_window[0] = moving_window[1];
            moving_window[1] = moving_window[2];
            moving_window[2] = moving_window[3];
        }
        println!("b_01_21: Successfully found {} positive sum deltas", count);

        count
    }
}

fn a_01_21(use_functional: bool) -> usize{
    let signal = parse_txt_file_to_int_vec("C:/Programming/advent_of_code_rust/input/day1a.txt");
    
    if use_functional{
        let count = signal.windows(2).filter(|x| x[0] < x[1]).count();
        println!("a_01_21: Successfully found {} positive sum deltas", count);
        count
    } else {
        let mut moving_window = [signal[0], signal[1]];
        let mut count: usize = 0;
        for element in 1..signal.len(){
            moving_window[1] = signal[element];
            if moving_window[0] < moving_window[1]{
                count += 1;
            }
            moving_window[0] = moving_window[1];
        }
        println!("a_01_21: Successfully found {} positive sum deltas", count);
        count
    }
}

fn main() -> std::io::Result<()> {
    a_01_21(false);
    b_01_21(false);    
    
    Ok(())
}


#[test]
fn test_b_01_21() {
    assert_eq!(b_01_21(true), 1724);
    assert_eq!(b_01_21(false), 1724);
}

#[test]
fn test_a_01_21() {
    assert_eq!(a_01_21(true), 1692);
    assert_eq!(a_01_21(false), 1692);
}

