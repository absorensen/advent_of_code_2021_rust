use std::fs::File;
use std::io::{prelude::*};


fn parse_txt_file_to_int_vec(path: &str) -> Vec<i32>{
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.lines().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn parse_txt_file_to_str_tokens(path: &str) -> Vec<Vec<String>>{
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // This code doesn't look nice to me, but I'm practicing iterators
    contents.lines()
        .map(|line| 
            line.split(" ")
                .map(|token| 
                    token.to_string())
                .collect()
            )
        .collect()
}

fn b_03_21(use_functional: bool) -> i32 {
    let status_values = parse_txt_file_to_str_tokens("C:/Programming/advent_of_code_rust/input/day3.txt");

    let number_of_digits = status_values[0][0].len();
    let number_of_lines = status_values.len() as u32;
    let mut valid_indices_scrubber = vec![0; number_of_lines as usize];
    let mut valid_indices_oxygen = vec![0; number_of_lines as usize];

    for line_index in 0..number_of_lines as usize {
        valid_indices_scrubber[line_index as usize] = line_index;
        valid_indices_oxygen[line_index as usize] = line_index;     
    }

    let mut valid_oxygen_count = number_of_lines;
    let mut valid_scrubber_count = number_of_lines;

    for bit_index in 0..number_of_digits{
        let mut most_common_oxygen = 0;
        let mut most_common_scrubber = 0;
        for line_index in 0..number_of_lines as usize {
            let mut char_iterator = status_values[line_index][0].chars();
            for _ in 0..bit_index{
                char_iterator.next();
            }
            let bit = char_iterator.next().unwrap().to_digit(10).unwrap();

            if bit == 1 && valid_indices_oxygen[line_index] != usize::MAX {
                most_common_oxygen += 1;
            }

            if bit == 1 && valid_indices_scrubber[line_index] != usize::MAX {
                most_common_scrubber += 1;
            }
        }

        if most_common_oxygen >= valid_oxygen_count - most_common_oxygen {
            most_common_oxygen = 1;
        } else {
            most_common_oxygen = 0;
        }

        if most_common_scrubber >= valid_scrubber_count - most_common_scrubber {
            most_common_scrubber = 1;
        } else {
            most_common_scrubber = 0;
        }

        for line_index in 0..number_of_lines as usize {
            let mut char_iterator = status_values[line_index][0].chars();
            for _ in 0..bit_index{
                char_iterator.next();
            }
            let bit = char_iterator.next().unwrap().to_digit(10).unwrap();

            if valid_oxygen_count > 1 && valid_indices_oxygen[line_index] != usize::MAX {
                if most_common_oxygen != bit {
                    valid_indices_oxygen[line_index] = usize::MAX;
                    valid_oxygen_count -= 1;
                }
            }

            if valid_scrubber_count > 1 && valid_indices_scrubber[line_index] != usize::MAX {
                if most_common_scrubber == bit {
                    valid_indices_scrubber[line_index] = usize::MAX;
                    valid_scrubber_count -= 1;
                }
            }
        }

        if valid_scrubber_count == 1 && valid_oxygen_count == 1 {
            break;
        }
    }

    let mut scrubber_bits = vec![0; number_of_digits];
    for line_index in 0..number_of_lines as usize {
        if valid_indices_scrubber[line_index] != usize::MAX {
            println!("Valid scrubber index: {}", line_index);
            let mut char_iterator = status_values[line_index][0].chars();
            for digit_index in 0..number_of_digits{
                scrubber_bits[digit_index] = char_iterator.next().unwrap().to_digit(10).unwrap();
            }
        } 
    }

    let mut oxygen_bits = vec![0; number_of_digits];
    for line_index in 0..number_of_lines as usize {
        if valid_indices_oxygen[line_index] != usize::MAX {
            println!("Valid oxygen index: {}", line_index);
            let mut char_iterator = status_values[line_index][0].chars();
            for digit_index in 0..number_of_digits{
                oxygen_bits[digit_index] = char_iterator.next().unwrap().to_digit(10).unwrap();
            }
        } 
    }

    let mut scrubber_rating = 0;
    let mut oxygen_rating = 0;

    for bit_index in 0..number_of_digits {
        if scrubber_bits[bit_index] == 1 {
            let move_by: usize = number_of_digits - bit_index - 1;
            scrubber_rating += 1 << move_by;
        }
        if oxygen_bits[bit_index] == 1 {
            let move_by: usize = number_of_digits - bit_index - 1;
            oxygen_rating += 1 << move_by;
        }
    }


    println!("b_03_21: Scrubber Rating: {} Oxygen Rating: {} Product: {}", scrubber_rating, oxygen_rating, scrubber_rating * oxygen_rating);
    scrubber_rating * oxygen_rating
}

fn a_03_21(use_functional: bool) -> u32 {
    let status_values = parse_txt_file_to_str_tokens("C:/Programming/advent_of_code_rust/input/day3.txt");

    let number_of_digits = status_values[0][0].len();
    let number_of_lines = status_values.len() as u32;
    let mut number_of_ones:Vec<u32> = vec![0; number_of_digits];
    for status in status_values {
        let mut char_iterator = status[0].chars();
        for index in 0..number_of_digits {
            let digit = char_iterator.next().unwrap().to_digit(10).unwrap();
            if digit == 1 {
                number_of_ones[index] += 1;
            }
        }
    }

    let mut gamma_bits:Vec<u32> = vec![0; number_of_digits];
    let mut epsilon_bits:Vec<u32> = vec![0; number_of_digits];
    for index in 0..number_of_digits {
        if number_of_ones[index] > number_of_lines / 2 { 
            gamma_bits[index] = 1; 
        }

        if number_of_ones[index] < number_of_lines / 2 { 
            epsilon_bits[index] = 1; 
        }
    }

    let mut gamma:u32 = 0;
    let mut epsilon:u32 = 0;

    for bit_index in 0..number_of_digits {
        if gamma_bits[bit_index] == 1 {
            let move_by: usize = number_of_digits - bit_index - 1;
            gamma += 1 << move_by;
        }
        if epsilon_bits[bit_index] == 1 {
            let move_by: usize = number_of_digits - bit_index - 1;
            epsilon += 1 << move_by;
        }
    }

    println!("a_03_21: Gamma: {} Epsilon: {} Product: {}", gamma, epsilon, gamma * epsilon);
    (gamma * epsilon) as u32
}

fn b_02_21(use_functional: bool) -> i32 {
    let commands = parse_txt_file_to_str_tokens("C:/Programming/advent_of_code_rust/input/day2.txt");

    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;
    let mut aim: i32 = 0;
    for command_index in 0..commands.len() {
        let direction = &commands[command_index][0];
        let magnitude = &commands[command_index][1].parse::<i32>().unwrap();

        match direction.as_str() {
            "forward" => {
                horizontal_position += magnitude;
                depth += aim * magnitude;
            }
            "down" => {aim += magnitude;}
            "up" => {aim -= magnitude;}
            _ => {}
        }
    }

    println!("a_02_21: Horizontal Postion: {} Aim: {} Depth: {}  Product: {}", horizontal_position, aim, depth, horizontal_position * depth);
    horizontal_position * depth
}

fn a_02_21(use_functional: bool) -> i32 {
    let commands = parse_txt_file_to_str_tokens("C:/Programming/advent_of_code_rust/input/day2.txt");

    let mut depth: i32 = 0;
    let mut horizontal_position: i32 = 0;
    for command_index in 0..commands.len() {
        let direction = &commands[command_index][0];
        let magnitude = &commands[command_index][1].parse::<i32>().unwrap();

        match direction.as_str() {
            "forward" => {horizontal_position += magnitude;}
            "down" => {depth += magnitude;}
            "up" => {depth -= magnitude;}
            _ => {}
        }
    }

    println!("a_02_21: Horizontal Postion: {}  Depth: {}  Product: {}", horizontal_position, depth, horizontal_position * depth);
    horizontal_position * depth
}

fn b_01_21(use_functional: bool) -> usize {
    let signal = parse_txt_file_to_int_vec("C:/Programming/advent_of_code_rust/input/day1.txt");
    
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
    let signal = parse_txt_file_to_int_vec("C:/Programming/advent_of_code_rust/input/day1.txt");
    
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
    a_03_21(false);
    b_03_21(false);    
    
    Ok(())
}

#[test]
fn test_b_02_21() {
    assert_eq!(b_02_21(true), 1408487760);
    assert_eq!(b_02_21(false), 1408487760);
}

#[test]
fn test_a_02_21() {
    assert_eq!(a_02_21(true), 1690020);
    assert_eq!(a_02_21(false), 1690020);
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

