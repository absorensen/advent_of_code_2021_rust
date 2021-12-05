#![allow(dead_code, unused_assignments, unused_variables)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let now = std::time::Instant::now();
    a_05_21(false);
    let elapsed_time = now.elapsed();
    println!("Running function a took {} microseconds.", elapsed_time.as_micros());

    let now = std::time::Instant::now();
    b_05_21(false);    
    let elapsed_time = now.elapsed();
    println!("Running function b took {} microseconds.", elapsed_time.as_micros());

    Ok(())
}

struct BingoBoardElement {
    value: i32,
    marked: bool,
}

struct BingoBoard {
    number_of_rows: usize,
    number_of_columns: usize,
    elements: Vec<BingoBoardElement>,
}

impl BingoBoard {
    #[inline(always)]
    fn mut_index(&mut self, row:usize, column:usize) -> &mut BingoBoardElement {
        &mut self.elements[row * self.number_of_columns + column]
    }

    #[inline(always)]
    fn index(&self, row:usize, column:usize) -> & BingoBoardElement {
        &self.elements[row * self.number_of_columns + column]
    }


    fn update(&mut self, number: i32) -> () {
        for row in 0..self.number_of_rows {
            for column in 0..self.number_of_columns {
                if self.index(row, column).value == number {
                    self.mut_index(row, column).marked = true;
                }
            }
        }
    } 

    #[inline(always)]
    fn has_won(&self) -> bool {
        // Check all rows
        for row in 0..self.number_of_rows {
            let mut correct = 0;
            for column in 0..self.number_of_columns {
                if self.index(row, column).marked {
                    correct += 1;
                }
            }
            if correct == self.number_of_columns {
                return true
            }
        }

        // Check all columns
        for column in 0..self.number_of_columns {
            let mut correct = 0;
            for row in 0..self.number_of_rows {
                if self.index(row, column).marked {
                    correct += 1;
                }
            }
            if correct == self.number_of_rows {
                return true
            }
        }

        false
    }

    #[inline(always)]
    fn calculate_score(&self) -> i32 {
        self.elements.iter()
            .map(|element| 
                    if element.marked { 0 } else { element.value }
                )
            .sum()
    }
}

struct BingoSetup {
    boards: Vec<BingoBoard>,
    drawn_numbers: Vec<i32>,
}

fn parse_txt_to_bingo_setup(path:&str) -> BingoSetup {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut boards: Vec<BingoBoard> = Vec::<BingoBoard>::new();
    let mut drawn_numbers: Vec<i32> = Vec::<i32>::new();
    let mut board_lines_read: usize = 5;
    let mut boards_read: usize = 0;
    let elements_per_row: usize = 5;
    for (index, line) in reader.lines().enumerate(){
        let line = line.unwrap();

        if index == 0 {
            drawn_numbers = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            continue;
        }
        
        if board_lines_read == 5 {
            board_lines_read = 0;
            continue;
        }
        
        if board_lines_read == 0 {
            boards.push(BingoBoard{number_of_rows : elements_per_row, number_of_columns : elements_per_row, elements:Vec::<BingoBoardElement>::new()});
            boards_read += 1;
        }

        let line_numbers = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut elements : Vec<BingoBoardElement> = line_numbers.iter().map(|x| BingoBoardElement{ value : *x , marked : false}).collect();
        boards[boards_read - 1].elements.append(&mut elements);
        board_lines_read += 1;
    }

   BingoSetup{boards, drawn_numbers}
}

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

fn create_enumerated_vector(size:u32) -> Vec<u32> {
    (0..size).collect::<Vec<_>>()
}

fn bitvector_to_base_10(bits:&Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    let number_of_digits = bits.len();
    for bit_index in 0..number_of_digits {
        if bits[bit_index] == 1 {
            let move_by: usize = number_of_digits - bit_index - 1;
            result += 1 << move_by;
        }
    }
    result
}

fn parse_txt_to_bit_message(path: &str) -> Vec<Vec<u32>> {
    let strings = parse_txt_file_to_str_tokens(path);
    let number_of_digits = strings[0][0].len();
    let number_of_lines = strings.len() as u32;

    let mut bit_message = Vec::<Vec<u32>>::new();
    bit_message.resize(number_of_lines as usize, Vec::<u32>::new());
    for line_index in 0..number_of_lines {
        let mut char_iterator = strings[line_index as usize][0].chars();
        bit_message[line_index as usize].resize(number_of_digits, u32::MAX);
        for bit_index in 0..number_of_digits{
            bit_message[line_index as usize][bit_index as usize] = char_iterator.next().unwrap().to_digit(10).unwrap();
        }
    }

    bit_message
}

fn get_valid_index_values(valid_indices: &Vec<u32>, bit_message: &Vec<Vec<u32>>) -> Vec<u32>{
    let number_of_lines = valid_indices.len();
    for line_index in 0..number_of_lines {
        if valid_indices[line_index] != u32::MAX {
            return bit_message[line_index].clone()
        } 
    }

    Vec::<u32>::new()
}

struct SteamMap {
    number_of_rows: usize,
    number_of_columns: usize,
    elements: Vec<i32>,
}

impl SteamMap {
    #[inline(always)]
    fn mut_index(&mut self, row:usize, column:usize) -> &mut i32 {
        &mut self.elements[row * self.number_of_columns + column]
    }

    #[inline(always)]
    fn index(&self, row:usize, column:usize) -> & i32 {
        &self.elements[row * self.number_of_columns + column]
    }

    fn print(&self) -> () {
        for row in 0..self.number_of_rows {
            for column in 0..self.number_of_columns {
                let element = self.index(row, column);
                if *element == -1 {
                    print!(".");
                } else {
                    print!("{}", *element);
                }
            }
            print!("\n");
        }
    }

    fn update(&mut self, start_x: usize, start_y: usize, stop_x: usize, stop_y: usize) -> () {
        let smallest_x = if start_x < stop_x { start_x } else { stop_x };
        let largest_x = if start_x < stop_x { stop_x } else { start_x };
        let smallest_y = if start_y < stop_y { start_y } else { stop_y };
        let largest_y = if start_y < stop_y { stop_y } else { start_y };
        if smallest_x != largest_x && smallest_y != largest_y {
            let mut row_index = start_y as i32;
            let mut column_index = start_x as i32;
            for row in smallest_y..largest_y+1 {
                let element = self.mut_index(row_index as usize, column_index as usize);
                if *element == -1 {
                    *element = 1;
                } else {
                    *element += 1;
                }

                if start_y < stop_y {
                    row_index += 1;
                } else {
                    row_index -= 1;
                }

                if start_x < stop_x {
                    column_index += 1;
                } else {
                    column_index -= 1;
                }


            }    
        } else if smallest_x == largest_x {
            for row in smallest_y..largest_y+1 {
                let element = self.mut_index(row, start_x);
                if *element == -1 {
                    *element = 1;
                } else {
                    *element += 1;
                }
            }    
        } else {
            for column in smallest_x..largest_x+1 {
                let element = self.mut_index(start_y, column);
                if *element == -1 {
                    *element = 1;
                } else {
                    *element += 1;
                }
            }
        }
    }

    fn simple_update(&mut self, start_x: usize, start_y: usize, stop_x: usize, stop_y: usize) -> () {
        if start_x != stop_x && start_y != stop_y {
            return;
        }

        let smallest_x = if start_x < stop_x { start_x } else { stop_x };
        let largest_x = if start_x < stop_x { stop_x } else { start_x };
        let smallest_y = if start_y < stop_y { start_y } else { stop_y };
        let largest_y = if start_y < stop_y { stop_y } else { start_y };
        if smallest_x == largest_x {
            for row in smallest_y..largest_y+1 {
                let element = self.mut_index(row, start_x);
                if *element == -1 {
                    *element = 1;
                } else {
                    *element += 1;
                }
            }    
        } else {
            for column in smallest_x..largest_x+1 {
                let element = self.mut_index(start_y, column);
                if *element == -1 {
                    *element = 1;
                } else {
                    *element += 1;
                }
            }
        }
    }

    fn count_line_overlaps(&self, threshold: i32) -> usize {
        let mut number_of_overlaps: usize = 0;
        for row in 0..self.number_of_rows {
            for column in 0..self.number_of_columns {
                let element = self.index(row, column);
                if threshold <= *element {
                    number_of_overlaps += 1;
                }
            }
        }
        number_of_overlaps
    }
}

fn parse_txt_file_to_steam_line_vec_usize(path: &str) -> Vec<usize> {
    let strings = parse_txt_file_to_str_tokens(path);
    let number_of_lines = strings.len();

    let mut lines = Vec::<usize>::new();
    lines.resize(number_of_lines * 4 as usize, 0);
    for line_index in 0..number_of_lines {
        let tokens: Vec<usize> = strings[line_index][0].split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        lines[line_index * 4] = tokens[0];
        lines[line_index * 4 + 1] = tokens[1];

        let tokens: Vec<usize> = strings[line_index][2].split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        lines[line_index * 4 + 2] = tokens[0];
        lines[line_index * 4 + 3] = tokens[1];
    }

    lines
}

fn b_05_21(use_functional: bool) -> i32 {
    let steam_lines = parse_txt_file_to_steam_line_vec_usize("C:/Programming/advent_of_code_rust/input/day5.txt");
    let max_value = steam_lines.iter().max().unwrap() + 1;
    let mut steam_map: SteamMap = initialize_steam_map(max_value as usize);
    let threshold = 2;

    let number_of_steam_lines = steam_lines.len() / 4;
    for steam_index in 0..number_of_steam_lines {
        let start_x = steam_lines[steam_index * 4 + 0];
        let start_y = steam_lines[steam_index * 4 + 1];
        let stop_x = steam_lines[steam_index * 4 + 2];
        let stop_y = steam_lines[steam_index * 4 + 3];

        steam_map.update(start_x, start_y, stop_x, stop_y);
    }

    let number_of_overlaps = steam_map.count_line_overlaps(threshold);
    // steam_map.print();

    println!("Problem B: Number of overlaps {} ", number_of_overlaps);
    number_of_overlaps as i32
}

fn initialize_steam_map(max_size: usize) -> SteamMap {
    let mut elements = Vec::<i32>::new();
    elements.resize(max_size * max_size, -1);
    SteamMap {number_of_rows: max_size, number_of_columns: max_size, elements: elements}
}

fn a_05_21(use_functional: bool) -> i32{
    let steam_lines = parse_txt_file_to_steam_line_vec_usize("C:/Programming/advent_of_code_rust/input/day5.txt");
    let max_value = steam_lines.iter().max().unwrap() + 1;
    let mut steam_map: SteamMap = initialize_steam_map(max_value as usize);
    let threshold = 2;

    let number_of_steam_lines = steam_lines.len() / 4;
    for steam_index in 0..number_of_steam_lines {
        let start_x = steam_lines[steam_index * 4 + 0];
        let start_y = steam_lines[steam_index * 4 + 1];
        let stop_x = steam_lines[steam_index * 4 + 2];
        let stop_y = steam_lines[steam_index * 4 + 3];

        steam_map.simple_update(start_x, start_y, stop_x, stop_y);
    }

    let number_of_overlaps = steam_map.count_line_overlaps(threshold);
    // steam_map.print();

    println!("Problem A: Number of overlaps {} ", number_of_overlaps);
    number_of_overlaps as i32
}


fn b_04_21(use_functional: bool) -> i32 {
    let bingo_boards = parse_txt_to_bingo_setup("C:/Programming/advent_of_code_rust/input/day4.txt");
    let mut boards = bingo_boards.boards;
    let drawn_numbers = bingo_boards.drawn_numbers;
    let mut winning_score: i32 = 0;
    let mut winners: Vec<bool> = Vec::<bool>::new();
    winners.resize(boards.len(), false);
    let mut number_of_winners = 0;
    let mut last_winner = 0;
    let mut last_draw = 0;
    for drawn_number in drawn_numbers {
        for board_index in 0..boards.len(){
            if winners[board_index] { continue; }

            let board = &mut boards[board_index];
            board.update(drawn_number);

            let has_won = board.has_won();

            if has_won {
                winning_score = board.calculate_score();
                winners[board_index] = true;
                number_of_winners += 1;
                if number_of_winners == boards.len() {
                    last_winner = board_index;
                }
            }
        }
        if number_of_winners == boards.len() {
            last_draw = drawn_number;
            winning_score *= drawn_number;
            break;
        }
    }

    println!("Winning Score {}, Last winner: {}, Last draw {} ", winning_score, last_winner, last_draw);
    winning_score
}



fn a_04_21(use_functional: bool) -> i32{
    let bingo_boards = parse_txt_to_bingo_setup("C:/Programming/advent_of_code_rust/input/day4.txt");
    let mut boards = bingo_boards.boards;
    let drawn_numbers = bingo_boards.drawn_numbers;
    let mut winner_found = false;
    let mut winning_score: i32 = 0;
    for drawn_number in drawn_numbers {
        for board_index in 0..boards.len(){
            let board = &mut boards[board_index];
            board.update(drawn_number);

            let has_won = board.has_won();

            if has_won {
                winning_score = board.calculate_score();
                winner_found = true;
                break;
            }
        }

        if winner_found {
            winning_score *= drawn_number;
            break;
        }
    }

    println!("Winning score was {}", winning_score);
    winning_score
}

fn b_03_21(use_functional: bool) -> u32 {
    let path = "C:/Programming/advent_of_code_rust/input/day3.txt";

    if use_functional {
        let bit_message = parse_txt_to_bit_message(path);
        let number_of_digits = bit_message[0].len();
        let number_of_lines = bit_message.len();

        let mut valid_indices_scrubber = create_enumerated_vector(number_of_lines as u32);
        let mut valid_indices_oxygen = create_enumerated_vector(number_of_lines as u32);
        let mut valid_oxygen_count:u32 = number_of_lines as u32;
        let mut valid_scrubber_count:u32 = number_of_lines as u32;

        for bit_index in 0..number_of_digits{
            let mut most_common_oxygen:u32 = 0;
            let mut most_common_scrubber:u32 = 0;
            for line_index in 0..number_of_lines as usize {
                let bit = bit_message[line_index][bit_index];
    
                if bit == 1 && valid_indices_oxygen[line_index] != u32::MAX {
                    most_common_oxygen += 1;
                }
    
                if bit == 1 && valid_indices_scrubber[line_index] != u32::MAX {
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
                let bit = bit_message[line_index][bit_index];
    
                if valid_oxygen_count > 1 && valid_indices_oxygen[line_index] != u32::MAX {
                    if most_common_oxygen != bit {
                        valid_indices_oxygen[line_index] = u32::MAX;
                        valid_oxygen_count -= 1;
                    }
                }
    
                if valid_scrubber_count > 1 && valid_indices_scrubber[line_index] != u32::MAX {
                    if most_common_scrubber == bit {
                        valid_indices_scrubber[line_index] = u32::MAX;
                        valid_scrubber_count -= 1;
                    }
                }
            }
    
            if valid_scrubber_count == 1 && valid_oxygen_count == 1 {
                break;
            }
        }

        let scrubber_bits:Vec<u32> = get_valid_index_values(&valid_indices_scrubber, &bit_message);
        let scrubber_rating:u32 = bitvector_to_base_10(&scrubber_bits);

        let oxygen_bits:Vec<u32> = get_valid_index_values(&valid_indices_oxygen, &bit_message);
        let oxygen_rating:u32 = bitvector_to_base_10(&oxygen_bits);
    
        println!("b_03_21: Scrubber Rating: {} Oxygen Rating: {} Product: {}", scrubber_rating, oxygen_rating, scrubber_rating * oxygen_rating);
        scrubber_rating * oxygen_rating

    } else {

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
}

fn a_03_21(use_functional: bool) -> u32 {
    let path = "C:/Programming/advent_of_code_rust/input/day3.txt";

    if use_functional {
        
        let bit_message = parse_txt_to_bit_message(path);
        let number_of_digits = bit_message[0].len();
        let number_of_lines = bit_message.len();
        let mut number_of_ones:Vec<u32> = vec![0; number_of_digits];

        for line_index in 0..number_of_lines {
            let bits = &bit_message[line_index];
            for bit_index in 0..number_of_digits {
                let bit = &bits[bit_index];
                if *bit == 1 {
                    number_of_ones[bit_index] += 1;
                }
            }
        }

        let halfway:u32 = number_of_lines as u32 / 2;
        let gamma_bits:Vec<u32> = number_of_ones.iter().map(|x| if x > &halfway {1} else {0}).collect();
        let epsilon_bits:Vec<u32> = number_of_ones.iter().map(|x| if x < &halfway {1} else {0}).collect();

        let gamma:u32 = bitvector_to_base_10(&gamma_bits);
        let epsilon:u32 = bitvector_to_base_10(&epsilon_bits);

        println!("a_03_21: Gamma: {} Epsilon: {} Product: {}", gamma, epsilon, gamma * epsilon);
        (gamma * epsilon) as u32

    } else {
        let status_values = parse_txt_file_to_str_tokens(path);

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





#[test]
fn test_b_04_21() {
    assert_eq!(b_04_21(true), 31755);
    assert_eq!(b_04_21(false), 31755);
}

#[test]
fn test_a_04_21() {
    assert_eq!(a_04_21(true), 6592);
    assert_eq!(a_04_21(false), 6592);
}

#[test]
fn test_b_03_21() {
    assert_eq!(b_03_21(true), 1370737);
    assert_eq!(b_03_21(false), 1370737);
}

#[test]
fn test_a_03_21() {
    assert_eq!(a_03_21(true), 775304);
    assert_eq!(a_03_21(false), 775304);
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

