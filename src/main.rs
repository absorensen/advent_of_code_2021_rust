#![allow(dead_code, unused_assignments, unused_variables)]

use std::fs::File;
use std::io::{prelude::*, BufReader};

use rayon::prelude::*;

fn main() -> std::io::Result<()> {
    let use_functional = true;

    let now = std::time::Instant::now();
    a_12_21(use_functional);
    let elapsed_time = now.elapsed();
    println!("Running function a took {} microseconds.", elapsed_time.as_micros());

    let now = std::time::Instant::now();
    b_12_21(use_functional);    
    let elapsed_time = now.elapsed();
    println!("Running function b took {} microseconds.", elapsed_time.as_micros());

    Ok(())
}

struct GraphNode {
    name: String,
    index: usize,
    connections: Vec<usize>,
    is_start: bool,
    is_end: bool,
    big_cave: bool,
}

struct Graph {
    nodes: Vec<GraphNode>,
    start_index: usize,
    end_index: usize,
}

impl Graph {

    fn generate_more_complicated_paths(&self) -> Vec<Vec<usize>> {
        let mut valid_paths = Vec::<Vec<usize>>::new();
        let mut processing_queue = Vec::<Vec<usize>>::new();

        let mut first_path = Vec::<usize>::new();
        first_path.push(self.start_index);
        processing_queue.push(first_path);

        while 0 < processing_queue.len() {
            let path: Vec<usize> = processing_queue.pop().unwrap();
            
            let mut path_found = false;
            let last_path_node_index = path[path.len()-1];
            let connections = &self.nodes[last_path_node_index].connections;
            for connection_index in 0..connections.len() {
                let connection = connections[connection_index];
                if self.is_valid_node_b(&path, connection) {
                    path_found = true;

                    let mut split_path = path.clone();
                    split_path.push(connection);

                    if self.is_complete_path(&split_path,self.end_index) {
                        valid_paths.push(split_path); 
                     } else {
                        processing_queue.push(split_path);
                     }
                } else {
                    if path_found && self.is_valid_node_b(&path, connection) {
                        let mut split_path = self.split(&path);
                        split_path.push(connection);
                        processing_queue.push(split_path);

                        if self.is_complete_path(&path, self.end_index) {
                            valid_paths.push(path.clone()); 
                        } else {
                            processing_queue.push(path.clone());
                        }
                    }
                }
            }
        } 
        

        valid_paths
    }

    fn generate_paths(&self) -> Vec<Vec<usize>> {
        let mut valid_paths = Vec::<Vec<usize>>::new();
        let mut processing_queue = Vec::<Vec<usize>>::new();

        let mut first_path = Vec::<usize>::new();
        first_path.push(self.start_index);
        processing_queue.push(first_path);

        while 0 < processing_queue.len() {
            let path: Vec<usize> = processing_queue.pop().unwrap();
            
            let mut path_found = false;
            let last_path_node_index = path[path.len()-1];
            let connections = &self.nodes[last_path_node_index].connections;
            for connection_index in 0..connections.len() {
                let connection = connections[connection_index];
                if self.is_valid_node(&path, connection) {
                    path_found = true;

                    let mut split_path = path.clone();
                    split_path.push(connection);

                    if self.is_complete_path(&split_path,self.end_index) {
                        valid_paths.push(split_path); 
                     } else {
                        processing_queue.push(split_path);
                     }
                } else {
                    if path_found && self.is_valid_node(&path, connection) {
                        let mut split_path = self.split(&path);
                        split_path.push(connection);
                        processing_queue.push(split_path);

                        if self.is_complete_path(&path, self.end_index) {
                            valid_paths.push(path.clone()); 
                        } else {
                            processing_queue.push(path.clone());
                        }
                    }
                }
            }
        } 
        

        valid_paths
    }

    fn is_match(&self, path: &Vec<usize>, other_path: &Vec<usize>) -> bool {
        if path.len() != other_path.len() {
            return false;
        }

        for node_index in 0..self.nodes.len() {
            if path[node_index] != other_path[node_index] {
                return false;
            }
        }

        true
    }

    fn split(&self, path: &Vec<usize>) -> Vec<usize> {
        path.clone()
    } 

    fn is_valid_node(&self, path: &Vec<usize>, new_node_index: usize) -> bool {
        if path.contains(&new_node_index) && !self.nodes[new_node_index].big_cave {
            return false
        }
        true
    }

    fn is_valid_node_b(&self, path: &Vec<usize>, new_node_index: usize) -> bool {
        if path.contains(&new_node_index) {
            if !self.nodes[new_node_index].big_cave && new_node_index != self.start_index && !self.contains_two_small_caves(path) {
                return true
            } else if path.contains(&new_node_index) && self.nodes[new_node_index].big_cave {
                return true
            } else {
                return false
            }
        }
        true
    }

    fn contains_two_small_caves(&self, path: &Vec<usize>) -> bool {
        let mut found_small_caves = Vec::<usize>::new();
        for node_index in 0..path.len() {
            let node = path[node_index];
            if found_small_caves.contains(&node) && node != self.end_index && node != self.start_index {
                return true
            }
            if !self.nodes[node].big_cave && node != self.end_index && node != self.start_index {
                found_small_caves.push(node);
            }
        }
        false
    }

    fn is_complete_path(&self, path: &Vec<usize>, last_node_index: usize) -> bool {
        let last_node = path.last().unwrap();
        if *last_node == last_node_index {
            return true;
        }

        false
    }
}


fn parse_txt_to_graph(path: &str) -> Graph {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut nodes = Vec::<GraphNode>::new();
    let lines: Vec<&str> = contents.lines().collect();

    let mut index_counter: usize = 0;
    let mut graph_start_index: usize = 0;
    let mut graph_end_index = 0;
    for line in lines {
        let tokens: Vec<&str> = line.split('-').collect();
        let start_name = tokens[0].to_string();
        let end_name = tokens[1].to_string();

        let start_index_result = nodes.iter().position(|node| *node.name == start_name);
        let mut start_index = 0;
        if start_index_result == None {
            let is_start = start_name == "start";
            let is_end = start_name == "end";
            nodes.push(GraphNode{name: start_name.clone(), index:index_counter, connections: Vec::<usize>::new(), is_start: is_start, is_end: is_end, big_cave: start_name.clone() == start_name.to_uppercase()});
            start_index = index_counter;
            if is_start { graph_start_index = start_index; }
            if is_end { graph_end_index = start_index; }
            index_counter += 1;
        } else {
            start_index = start_index_result.unwrap();
        }

        let end_index_result = nodes.iter().position(|node| *node.name == end_name);
        let mut end_index = 0;
        if end_index_result == None {
            let is_start = end_name == "start";
            let is_end = end_name == "end";
            nodes.push(GraphNode{name: end_name.clone(), index:index_counter, connections: Vec::<usize>::new(), is_start: is_start, is_end: is_end, big_cave: end_name.clone() == end_name.to_uppercase()});
            end_index = index_counter;
            if is_start { graph_start_index = end_index; }
            if is_end { graph_end_index = end_index; }
            index_counter += 1;
        } else {
            end_index = end_index_result.unwrap();
        }

        nodes[start_index].connections.push(end_index);
        nodes[end_index].connections.push(start_index);

    }


    Graph{nodes:nodes, start_index:graph_start_index, end_index:graph_end_index}
}

fn b_12_21(use_functional:bool) -> usize {
    let mut graph = parse_txt_to_graph("C:/Programming/advent_of_code_rust/input/day12.txt");
    let paths = graph.generate_more_complicated_paths();

    let number_of_paths = paths.len();
    println!("b_12_21: Number of paths: {}", number_of_paths);
    number_of_paths
}

fn a_12_21(use_functional:bool) -> u64 {
    let mut graph = parse_txt_to_graph("C:/Programming/advent_of_code_rust/input/day12.txt");
    let paths = graph.generate_paths();

    let number_of_paths = paths.len();
    println!("a_12_21: Number of paths: {}", number_of_paths);
    number_of_paths as u64
}

struct DumboOctopusMap {
    number_of_rows: usize,
    number_of_columns: usize,
    elements: Vec<u32>,
    flashed: Vec<bool>,
}

impl DumboOctopusMap {
    #[inline(always)]
    fn mut_index(&mut self, row:usize, column:usize) -> &mut u32 {
        &mut self.elements[row * self.number_of_columns + column]
    }

    #[inline(always)]
    fn index(&self, row:usize, column:usize) -> & u32 {
        &self.elements[row * self.number_of_columns + column]
    }

    fn round_step_1(&mut self) -> () {
        for index in 0..self.number_of_columns*self.number_of_rows {
            self.elements[index] += 1;
            self.flashed[index] = false;
        }
    }

    fn round_step_3(&mut self) -> () {
        for index in 0..self.number_of_columns*self.number_of_rows {
            if self.flashed[index] {
                self.elements[index] = 0;
            }
            self.flashed[index] = false;
        }
    }

    

    fn print(&self) -> () {
        for row in 0..self.number_of_rows {
            for column in 0..self.number_of_columns {
                let element = self.index(row, column);
                print!("{}", *element);
            }
            print!("\n");
        }
    }

    fn round_step_2(&mut self) -> u64 {
        let mut number_of_flashes = 0;
        for row_index in 0..self.number_of_rows {
            for column_index in 0..self.number_of_columns {
                if !self.flashed[row_index * self.number_of_columns + column_index] && *self.index(row_index, column_index) > 9 {
                    self.flashed[row_index * self.number_of_columns + column_index] = true;
                    number_of_flashes += 1;

                    // 
                    // Row - 1
                    //
                    if 0 < row_index && 0 < column_index { 
                        *self.mut_index(row_index - 1, column_index - 1) += 1; 
                    }
                    
                    if 0 < row_index { 
                        *self.mut_index(row_index - 1, column_index) += 1; 
                    }
                    
                    if 0 < row_index && column_index < self.number_of_columns - 1 { 
                        *self.mut_index(row_index - 1, column_index + 1) += 1; 
                    }
                    
                    // 
                    // Row
                    //
                    if 0 < column_index { 
                        *self.mut_index(row_index, column_index - 1) += 1; 
                    }
                    
                    if column_index < self.number_of_columns - 1 { 
                        *self.mut_index(row_index, column_index + 1) += 1; 
                    }


                   // 
                    // Row + 1
                    //
                    if row_index < self.number_of_rows - 1 && 0 < column_index { 
                        *self.mut_index(row_index + 1, column_index - 1) += 1; 
                    }
                    
                    if row_index < self.number_of_rows - 1 { 
                        *self.mut_index(row_index + 1, column_index) += 1; 
                    }
                    
                    if row_index < self.number_of_rows - 1 && column_index < self.number_of_columns - 1 { 
                        *self.mut_index(row_index + 1, column_index + 1) += 1; 
                    }
                }
            }
        }

        number_of_flashes
    }


}

fn parse_dumbo_octopus_input(path:&str) -> DumboOctopusMap {
    const RADIX: u32 = 10;


    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<&str> = contents.lines().collect();
    let number_of_columns: usize = lines[0].chars().count();
    let number_of_rows: usize = lines.len();

    let mut elements = Vec::<u32>::new();

    for line in lines {
        elements.append(& mut line.chars().map(|c| c.to_digit(RADIX).unwrap()).collect::<Vec<u32>>());

    }

    let mut flashed:Vec<bool> = Vec::<bool>::new();
    flashed.resize(elements.len(), false);

    DumboOctopusMap{number_of_rows:number_of_rows, number_of_columns:number_of_columns, elements:elements, flashed:flashed}
}

fn b_11_21(use_functional:bool) -> usize {
    let mut octopi = parse_dumbo_octopus_input("C:/Programming/advent_of_code_rust/input/day11.txt");
    let number_of_steps = 10000;
    let mut number_of_flashes = 0;
    let mut first_step_with_all_flash = 0;

    for step_index in 0..number_of_steps {
        let mut total_flashes_this_step = 0;
        octopi.round_step_1();


        let mut change_found = true;
        while change_found {
            let new_flashes = octopi.round_step_2();
            change_found = new_flashes != 0;
            number_of_flashes += new_flashes;
            total_flashes_this_step += new_flashes;
        }
        
        octopi.round_step_3();

        if total_flashes_this_step == (octopi.number_of_rows * octopi.number_of_columns) as u64 {
            first_step_with_all_flash = step_index + 1;
            break;
        }
    }

    println!("b_11_21: First step with all flash: {}", first_step_with_all_flash);
    first_step_with_all_flash
}

fn a_11_21(use_functional:bool) -> u64 {
    let mut octopi = parse_dumbo_octopus_input("C:/Programming/advent_of_code_rust/input/day11.txt");
    let number_of_steps = 100;
    let mut number_of_flashes = 0;

    for step_index in 0..number_of_steps {

        octopi.round_step_1();


        let mut change_found = true;
        while change_found {
            let new_flashes = octopi.round_step_2();
            change_found = new_flashes != 0;
            number_of_flashes += new_flashes;
        }

        octopi.round_step_3();
    }

    println!("a_11_21: Number of flashes: {}", number_of_flashes);
    number_of_flashes
}


fn parse_syntax_checker_input (path: &str) -> Vec<Vec<char>> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<&str> = contents.lines().collect();
    let number_of_columns: usize = lines[0].chars().count();
    let number_of_rows: usize = lines.len();

    let mut input = Vec::<Vec<char>>::new();
    for line in lines {
        let chars:Vec<char> = line.chars().collect();
        input.push(chars);
    }

    input
}

fn is_opening_char(candidate : char) -> bool {
    match candidate {
        '(' => return true,
        '[' => return true,
        '{' => return true,
        '<' => return true,
        _   => return false,
    }
}

fn open_close_matcher(opening_char: char, new_char:char) -> bool {
    match opening_char {
        '(' => if new_char == ')' {return true;} else {return false;},
        '[' => if new_char == ']' {return true;} else {return false;},
        '{' => if new_char == '}' {return true;} else {return false;},
        '<' => if new_char == '>' {return true;} else {return false;},
        _ => return false,
    }
}

fn compute_syntax_error_score(offending_chars: Vec<char>) -> usize {
    let mut score = 0;

    for char in offending_chars {
        match char {
            ')' => score += 3,
            ']' => score += 57,
            '}' => score += 1197,
            '>' => score += 25137,
            _   => println!("Found invalid offending char!"),
        }   
    }

    score

}

fn get_autocomplete_scores(opening_chars: &mut Vec<char>) -> usize {
    let mut score = 0;
    while 0 < opening_chars.len() {
        let opening_char = opening_chars.pop().unwrap();
        match opening_char {
            '(' => score = score * 5 + 1 ,
            '[' => score = score * 5 + 2,
            '{' => score = score * 5 + 3 ,
            '<' => score = score * 5 + 4 ,
            _ => score += 0 ,
        }

    }

    score
}

fn b_10_21(use_functional:bool) -> usize {
    let input = parse_syntax_checker_input("C:/Programming/advent_of_code_rust/input/day10.txt");

    let number_of_lines = input.len();
    let mut offending_chars: Vec<char> = Vec::<char>::new();
    let mut incomplete_line_scores: Vec<usize> = Vec::<usize>::new();
    for line_index in 0..number_of_lines {
        let line = &input[line_index];
        let mut stack = Vec::<char>::new();
        let mut corrupted_line = false;
        
        for char in line {
            if 0 < stack.len() {
                let top_stack_char = stack.pop().unwrap();
                if is_opening_char(*char) {
                    stack.push(top_stack_char);
                    stack.push(*char);
                } else if !open_close_matcher(top_stack_char, *char) {
                    offending_chars.push(*char);
                    corrupted_line = true;
                    break;
                }
            } else {
                if is_opening_char(*char) {
                    stack.push(*char); 
                } else {
                    offending_chars.push(*char);
                    corrupted_line = true;
                    break;
                }
            }
        }

        if !corrupted_line && 0 < stack.len() {
            incomplete_line_scores.push(get_autocomplete_scores(&mut stack))
        }
    }
    incomplete_line_scores.sort();
    let scores = incomplete_line_scores[incomplete_line_scores.len() / 2];

    println!("b_10_21: Score: {}", scores);
    scores
}

fn a_10_21(use_functional:bool) -> usize {
    let input = parse_syntax_checker_input("C:/Programming/advent_of_code_rust/input/day10.txt");

    let number_of_lines = input.len();
    let mut offending_chars: Vec<char> = Vec::<char>::new();
    for line_index in 0..number_of_lines {
        let line = &input[line_index];
        let mut stack = Vec::<char>::new();
        let mut corrupted_line = false;
        
        for char in line {
            if stack.len() > 0 {
                let top_stack_char = stack.pop().unwrap();
                if is_opening_char(*char) {
                    stack.push(top_stack_char);
                    stack.push(*char);
                } else if !open_close_matcher(top_stack_char, *char) {
                    offending_chars.push(*char);
                    corrupted_line = true;
                    break;
                }
            } else {
                if is_opening_char(*char) {
                    stack.push(*char); 
                } else {
                    offending_chars.push(*char);
                    corrupted_line = true;
                    break;
                }
            }
        }
    }

    let scores = compute_syntax_error_score(offending_chars);

    println!("a_10_21: Score: {}", scores);
    scores
}

struct HeightMap {
    number_of_rows: usize,
    number_of_columns: usize,
    elements: Vec<i32>,
    local_minima: Vec<(i32,usize,usize)>,
    basins: Vec<Vec<(usize,usize)>>,
}

impl HeightMap {
    #[inline(always)]
    fn mut_index(&mut self, row:usize, column:usize) -> &mut i32 {
        &mut self.elements[row * self.number_of_columns + column]
    }

    #[inline(always)]
    fn index(&self, row:usize, column:usize) -> & i32 {
        &self.elements[row * self.number_of_columns + column]
    }

    fn compute_basins(&mut self) -> () {
        let mut basins_queue= Vec::<(usize, usize)>::new();
        for row in 0..self.number_of_rows {
            for column in 0..self.number_of_columns {
                let central_value = self.index(row, column);
                if *central_value == 9{ continue; }
                basins_queue.push((row,column));
            }
        }

        self.compute_local_minima();
        
        let mut basins = Vec::<Vec<(usize,usize)>>::new();
        for (height, row, column) in &self.local_minima {
            let mut new_basin = Vec::<(usize,usize)>::new();
            new_basin.push((*row, *column));
            basins.push(new_basin);
        }

        let mut loops_since_change = 0;
        loop {
            for basin_queue_index in 0..basins_queue.len(){
                let (row, column) = basins_queue[basin_queue_index];
                let mut found = false;

                for basin_index in 0..basins.len(){
                    if found {break;}
                    for (basin_row, basin_column) in &basins[basin_index]{
                        if basins[basin_index].contains(&(row, column)){
                            found = true; // Was already a basin
                        }
                        break;
                    }
                }

                if !found && column > 0 {
                    for basin_index in 0..basins.len(){
                        if found {break;}
                        for (basin_row, basin_column) in &basins[basin_index]{
                            if row == *basin_row && column-1 == *basin_column {
                                found = true;
                                if !basins[basin_index].contains(&(row, column)){
                                    basins[basin_index].push((row,column));
                                }
                                break;
        
                            }
                        }
                    }
                }
    
                if !found && column < self.number_of_columns - 1 {
                    for basin_index in 0..basins.len(){
                        if found {break;}
                        for (basin_row, basin_column) in &basins[basin_index]{
                            if row == *basin_row && column + 1 == *basin_column {
                                found = true;
                                if !basins[basin_index].contains(&(row, column)){
                                    basins[basin_index].push((row,column));
                                }
                                break;
        
                            }
                        }
                    }
                }
    
                if !found && row > 0 {
                    for basin_index in 0..basins.len(){
                        if found {break;}
                        for (basin_row, basin_column) in &basins[basin_index]{
                            if row - 1 == *basin_row && column == *basin_column {
                                found = true;
                                if !basins[basin_index].contains(&(row, column)){
                                    basins[basin_index].push((row,column));
                                }
                                break;
        
                            }
                        }
                    }
                }
    
                if !found && row < self.number_of_rows - 1 {
                    for basin_index in 0..basins.len(){
                        if found {break;}
                        for (basin_row, basin_column) in &basins[basin_index]{
                            if row + 1 == *basin_row && column == *basin_column {
                                found = true;
                                if !basins[basin_index].contains(&(row, column)){
                                    basins[basin_index].push((row,column));
                                }
                                break;
        
                            }
                        }
                    }
                }
    
                if found {
                    basins_queue.remove(basin_queue_index);
                    loops_since_change = 0;
                    break;
                }
    
            }

            loops_since_change += 1;
            if basins_queue.len() < 1 || loops_since_change > 10 {
                break;
            }
        }


        self.basins = basins;
    }

    fn compute_local_minima(&mut self) -> () {
        let mut local_minima = Vec::<(i32, usize, usize)>::new();
        for row in 0..self.number_of_rows {
            for column in 0..self.number_of_columns {
                let mut valid_directions = 0;
                let mut smaller_than_directions = 0;
                let central_value = self.index(row, column);
                if column > 0 {
                    valid_directions += 1;
                    if self.index(row, column-1) > central_value {
                        smaller_than_directions += 1;
                    }
                }

                if column < self.number_of_columns - 1 {
                    valid_directions += 1;
                    if self.index(row, column + 1) > central_value {
                        smaller_than_directions += 1;
                    }
                }

                if row > 0 {
                    valid_directions += 1;
                    if self.index(row-1, column) > central_value {
                        smaller_than_directions += 1;
                    }
                }

                if row < self.number_of_rows - 1 {
                    valid_directions += 1;
                    if self.index(row + 1, column) > central_value {
                        smaller_than_directions += 1;
                    }
                }

                if smaller_than_directions == valid_directions {
                    local_minima.push((central_value.clone(), row, column));
                }
            }
        }

        self.local_minima = local_minima;
    }

    fn compute_basin_scores(&self) -> i64 {
        let mut biggest_values = Vec::<i64>::new();
        biggest_values.resize(3, 0);

        for basin_index in 0..self.basins.len() {
            let basin_size = self.basins[basin_index].len() as i64;
            let minimum_value = biggest_values.iter().min().unwrap();
            let minimum_index = biggest_values.iter().position(|value| value == minimum_value).unwrap();
            if minimum_value < &basin_size {
                biggest_values[minimum_index] = basin_size
            }
        }
  
        biggest_values[0] * biggest_values[1] * biggest_values[2] 
    }

    fn compute_risk(&self) -> i64 {
        let mut risk = 0;
        for (height, row, column) in &self.local_minima {
            risk += 1 + *height as i64;
        }
        risk
    }
}

fn parse_txt_file_to_height_map(path: &str) -> HeightMap {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<&str> = contents.lines().collect();
    let number_of_columns: usize = lines[0].chars().count();
    let number_of_rows: usize = lines.len();

    let mut elements = Vec::<i32>::new();
    for line in lines {
        for char in line.chars() {
            elements.push(char as i32 - 0x30);
        }
    }
    let local_minima = Vec::<(i32, usize, usize)>::new();
    let basins = Vec::<Vec<(usize, usize)>>::new();

    HeightMap{number_of_rows:number_of_rows, number_of_columns:number_of_columns, elements:elements, local_minima:local_minima, basins:basins}
}

fn b_09_21(use_functional:bool) -> i64 {
    let mut height_map = parse_txt_file_to_height_map("C:/Programming/advent_of_code_rust/input/day9.txt");
    height_map.compute_basins();
    let scores = height_map.compute_basin_scores();

    println!("b_09_21: Score: {}", scores);
    scores
}

fn a_09_21(use_functional:bool) -> i64 {
    let mut height_map = parse_txt_file_to_height_map("C:/Programming/advent_of_code_rust/input/day9.txt");
    height_map.compute_local_minima();
    let risk_score = height_map.compute_risk();

    println!("a_09_21: Risk score: {}", risk_score);
    risk_score
}



#[derive(Clone, Copy, PartialEq)]
struct Signal {
    wires: i32,
}

impl Signal {

    fn add_signal(&mut self, signal: i32){
        self.wires |= 1 << signal;
    }

    fn active_signals(&self) -> i32 {
        self.wires.count_ones() as i32
    }

    fn contains_all(&self, other_signal: i32) -> bool {
        let new_values = other_signal & self.wires;
        let number_of_ones = self.wires.count_ones(); 
        let new_number_of_ones = new_values.count_ones();
        new_number_of_ones == number_of_ones
    }

    fn signal_with_bits_removed(&self, other_signal: i32) -> Signal {
        Signal{wires:(!other_signal) & self.wires}
    }
}

fn parse_signals_and_outputs(path: &str) -> (Vec<Vec<Signal>>, Vec<Vec<Signal>>) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let lines: Vec<&str> = contents.lines().collect();

    let mut signals : Vec<Vec<Signal>> = Vec::<Vec<Signal>>::new();
    signals.resize(lines.len(), Vec::<Signal>::new() );
    let mut outputs : Vec<Vec<Signal>> = Vec::<Vec<Signal>>::new();
    outputs.resize(lines.len(), Vec::<Signal>::new() );
    let mut line_index = 0;
    let a_int: i32 = 'a' as i32;
    for line in lines {
        let halves : Vec<&str> = line.split('|').collect();
        let signals_tokens: Vec<&str> = halves[0].split_whitespace().collect();
        let mut signal_index = 0;
        
        signals[line_index].resize(signals_tokens.len(), Signal{wires: 0});
        for signals_token in signals_tokens {
            let signal_chars: Vec<char> = signals_token.chars().collect();
            for signal_wire in signal_chars {
                signals[line_index][signal_index].add_signal(signal_wire as i32 - a_int);
            }

            signal_index += 1;
        }
        
        let outputs_tokens: Vec<&str> = halves[1].split_whitespace().collect();
        let mut output_index = 0;
        outputs[line_index].resize(outputs_tokens.len(), Signal{wires: 0});
        for outputs_token in outputs_tokens {
            let outputs_chars: Vec<char> = outputs_token.chars().collect();
            for signal_wire in outputs_chars {
                outputs[line_index][output_index].add_signal(signal_wire as i32 - a_int);
            }

            output_index += 1;
        }

        line_index += 1;
    }


    (signals, outputs)

}

fn b_08_21(use_functional:bool) -> i64 {
    let (all_signals, all_outputs) = parse_signals_and_outputs("C:/Programming/advent_of_code_rust/input/day8.txt");

    let mut global_sum = 0;
    for all_signals_index in 0..all_signals.len(){
        let signals = &all_signals[all_signals_index];
        let outputs = &all_outputs[all_signals_index];
        let mut signal_map: [Signal; 10] = [Signal{wires:0}; 10];

        for digit in 0..10 {
            for signal_index in 0..signals.len() {
                let signal = signals[signal_index];
                let signal_size = signal.active_signals();
                if signal_size == 2 {
                    if signal_map[1].wires == 0 {
                        signal_map[1] = signal.clone();
                    }
                } else if signal_size == 4 {
                    if signal_map[4].wires == 0 {
                        signal_map[4] = signal.clone();
                    }
                } else if signal_size == 3{
                    if signal_map[7].wires == 0 {
                        signal_map[7] = signal.clone();
                    }
                } else if signal_size == 7 {
                    if signal_map[8].wires == 0 {
                        signal_map[8] = signal.clone();
                    }
                } else if signal_size == 5 {
                    // case 3
                    if signal_map[1].wires != 0 && signal_map[1].contains_all(signal.wires) {
                        if signal_map[3].wires == 0 {
                            signal_map[3] = signal.clone();
                        }
                    
                    // case 5
                    } else if signal_map[4].wires != 0 && signal_map[1].wires != 0 && signal_map[3].wires != 0 && signal_map[4].signal_with_bits_removed(signal_map[1].wires).contains_all(signal.wires) {
                        if signal_map[5].wires == 0 {
                            signal_map[5] = signal.clone();
                        }
                    
                    // case 2
                    } else if  signal_map[4].wires != 0 && signal_map[1].wires != 0 && signal_map[4].wires != 0 && !signal_map[1].contains_all(signal.wires) && !(signal_map[4].signal_with_bits_removed(signal_map[1].wires).contains_all(signal.wires)) {
                        if signal_map[2].wires == 0 {
                            signal_map[2] = signal.clone();
                        }
                    }
    
                } else if signal_size == 6 {
                    // case 9
                    if signal_map[4].wires != 0 && signal_map[5].wires != 0 && signal_map[4].contains_all(signal.wires) && signal_map[5].contains_all(signal.wires){
                        if signal_map[9].wires == 0 {
                            signal_map[9] = signal.clone();
                        }
                    
                    // case 0
                    } else if signal_map[4].wires != 0 && signal_map[5].wires != 0 && signal_map[9].wires != 0 && !signal_map[5].contains_all(signal.wires) && !signal_map[4].contains_all(signal.wires) {
                        if signal_map[0].wires == 0 {
                            signal_map[0] = signal.clone();
                        }
                    
                    // case 6
                    } else if signal_map[4].wires != 0 && signal_map[5].wires != 0 && signal_map[0].wires != 0 &&signal_map[5].contains_all(signal.wires) && !signal_map[4].contains_all(signal.wires) {
                        if signal_map[6].wires == 0 {
                            signal_map[6] = signal.clone();
                        }
                    }
                }
            }
        }

        let mut sum = 0;
        for output_index in 0..outputs.len() {
            let signal = outputs[output_index];
            let signal_size = signal.active_signals();

            if signal.wires == signal_map[0].wires && signal.active_signals() == 6{
                sum = sum * 10 + 0;
            } else if signal.wires == signal_map[1].wires && signal_size == 2{
                sum = sum * 10 + 1;
            } else if signal.wires == signal_map[2].wires && signal_size == 5{
                sum = sum * 10 + 2;
            } else if signal.wires == signal_map[3].wires && signal_size == 5{
                sum = sum * 10 + 3;
            } else if signal.wires == signal_map[4].wires && signal_size == 4{
                sum = sum * 10 + 4;
            } else if signal.wires == signal_map[5].wires && signal_size == 5{
                sum = sum * 10 + 5;
            } else if signal.wires == signal_map[6].wires && signal_size == 6{
                sum = sum * 10 + 6;
            } else if signal.wires == signal_map[7].wires && signal_size == 3{
                sum = sum * 10 + 7;
            } else if signal.wires == signal_map[8].wires && signal_size == 7{
                sum = sum * 10 + 8;
            } else if signal.wires == signal_map[9].wires && signal_size == 6{
                sum = sum * 10 + 9;
            }

        }
        global_sum += sum;
    }



    println!("b_08_21: Global sum: {}", global_sum);
    global_sum
}

fn a_08_21(use_functional:bool) -> i64 {
    let (signals, outputs) = parse_signals_and_outputs("C:/Programming/advent_of_code_rust/input/day8.txt");
    let mut signal_lengths: [i32; 9] = [0; 9];
    signal_lengths[1] = 2;
    signal_lengths[4] = 4;
    signal_lengths[7] = 3;
    signal_lengths[8] = 7;

    let mut valid_outputs_counter = 0;


    for output in outputs{
        for signal in output {
            let signal_size = signal.active_signals();

            if signal_size == signal_lengths[1] {
                valid_outputs_counter += 1;
                continue;
            }

            if signal_size == signal_lengths[4] {
                valid_outputs_counter += 1;
                continue;
            }

            if signal_size == signal_lengths[7] {
                valid_outputs_counter += 1;
                continue;
            }

            if signal_size == signal_lengths[8] {
                valid_outputs_counter += 1;
                continue;
            }
        }
    }
    println!("a_08_21: Valid outputs counter: {}", valid_outputs_counter);
    valid_outputs_counter
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
                if *element == 0 {
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
            let modifier_row = if start_y < stop_y { 1 } else { -1 };
            let modifier_column = if start_x < stop_x { 1 } else { -1 };
        
            for row in smallest_y..largest_y+1 {
                *self.mut_index(row_index as usize, column_index as usize) += 1;
                row_index += modifier_row;
                column_index += modifier_column;
            }    
        
        } else if smallest_x == largest_x {
            
            for row in smallest_y..largest_y+1 {
                *self.mut_index(row, start_x) += 1;
            }    
        
        } else {
        
            for column in smallest_x..largest_x+1 {
                *self.mut_index(start_y, column) += 1;
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
                *self.mut_index(row, start_x) += 1;
            }    
        } else {
            for column in smallest_x..largest_x+1 {
                *self.mut_index(start_y, column) += 1;
            }
        }
    }

    fn count_line_overlaps(&self, threshold: i32) -> usize {
        self.elements.par_iter().filter(|element| threshold <= **element).count()
    }
}

fn initialize_steam_map(max_size: usize) -> SteamMap {
    let mut elements = Vec::<i32>::new();
    elements.resize(max_size * max_size, 0);
    SteamMap {number_of_rows: max_size, number_of_columns: max_size, elements: elements}
}

fn parse_txt_file_to_steam_line_vec_usize(path: &str) -> Vec<usize> {
    let strings = parse_txt_file_to_str_tokens(path);
    let number_of_lines = strings.len();

    let mut lines = Vec::<usize>::new();
    lines.resize(number_of_lines * 4 as usize, 0);
    for line_index in 0..number_of_lines {
        let first_tokens: Vec<usize> = strings[line_index][0].split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let second_tokens: Vec<usize> = strings[line_index][2].split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        lines[line_index * 4 + 0] = first_tokens[0];
        lines[line_index * 4 + 1] = first_tokens[1];
        lines[line_index * 4 + 2] = second_tokens[0];
        lines[line_index * 4 + 3] = second_tokens[1];
    }

    lines
}

fn parse_txt_file_to_lantern_fish(path:&str) -> Vec<usize> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents.split(',').map(|s| s.parse::<usize>().unwrap()).collect()
}

fn int_vec_to_occurrences(buckets: usize, fish: &Vec<usize>) -> Vec<usize> {
    let mut occurences = Vec::<usize>::new();
    occurences.resize(buckets, 0);

    for fish_index in 0..fish.len(){
        occurences[fish[fish_index as usize] as usize] += 1;
    }

    occurences
}

fn parse_txt_file_to_crab_horizontals(path:&str) -> (Vec<i64>, i64, i64) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tokens: Vec::<&str> = contents.split(',').collect();
    let mut crab_vector = Vec::<i64>::new();
    crab_vector.resize(tokens.len(), 0);

    let mut minimum_position : i64 = i64::MAX;
    let mut maximum_position : i64 = i64::MIN;
    for token_index in 0..tokens.len() {
         crab_vector[token_index] = tokens[token_index].parse::<i64>().unwrap();
         minimum_position = if crab_vector[token_index] < minimum_position { crab_vector[token_index] } else { minimum_position };
         maximum_position = if maximum_position < crab_vector[token_index] { crab_vector[token_index] } else { maximum_position };
    }

    (crab_vector, maximum_position, minimum_position)
}



fn nth_triangle_number(input: i64) -> i64 {
    (input * input + input) >> 1
}

fn b_07_21(use_functional:bool) -> i64 {
    let (crabs, max_position, min_position) = parse_txt_file_to_crab_horizontals("C:/Programming/advent_of_code_rust/input/day7.txt");

    if use_functional {
        (min_position..=max_position)
            .into_par_iter()
            .map(|position_index| 
                crabs
                    .iter()
                    .map(|x| nth_triangle_number((*x - position_index).abs()))
                    .sum())
                    .min()
                    .unwrap()
    } else {
        let mut total_fuel_cost : i64 = 0;
        let mut best_fuel_cost : i64 = i64::MAX;
        let mut best_fuel_position : i64 = 0;
        
        for position_index in min_position..max_position+1{
            total_fuel_cost = crabs.iter().map(|x| nth_triangle_number((*x - position_index).abs())).sum();

            if total_fuel_cost < best_fuel_cost {
                best_fuel_cost = total_fuel_cost;
                best_fuel_position = position_index;
            }
        }

        println!("b_07_21: Total fuel cost: {} at position: {} ", best_fuel_cost, best_fuel_position);
        best_fuel_cost
    }
}

fn a_07_21(use_functional:bool) -> i64 {
    let (crabs, max_position, min_position) = parse_txt_file_to_crab_horizontals("C:/Programming/advent_of_code_rust/input/day7.txt");

    if use_functional {
        (min_position..=max_position).into_par_iter().map(|position_index| crabs.iter().map(|x| (*x - position_index).abs()).sum()).min().unwrap()
    } else {
        let mut total_fuel_cost : i64 = 0;
        let mut best_fuel_cost : i64 = i64::MAX;
        let mut best_fuel_position : i64 = 0;
        for position_index in min_position..max_position+1{
            total_fuel_cost = crabs.iter().map(|x| (*x - position_index).abs()).sum();
    
            if total_fuel_cost < best_fuel_cost {
                best_fuel_cost = total_fuel_cost;
                best_fuel_position = position_index;
            }
        }
    
        println!("b_07_21: Total fuel cost: {} at position: {} ", best_fuel_cost, best_fuel_position);
        best_fuel_cost
    }
}

fn b_06_21(use_functional: bool) -> usize {

    if use_functional {
        let lantern_fish = parse_txt_file_to_lantern_fish("C:/Programming/advent_of_code_rust/input/day6.txt");
        let buckets = 9;
        let mut lantern_fish_occurrences = int_vec_to_occurrences(buckets, &lantern_fish);
        let simulate_number_of_days = 256;

        for day in 0..simulate_number_of_days {
            lantern_fish_occurrences.rotate_left(1);
            lantern_fish_occurrences[6] += lantern_fish_occurrences[8];
        }
        
        lantern_fish_occurrences.iter().sum()        

    } else {
        let lantern_fish = parse_txt_file_to_lantern_fish("C:/Programming/advent_of_code_rust/input/day6.txt");
        let buckets = 9;
        let mut lantern_fish_occurrences = int_vec_to_occurrences(buckets, &lantern_fish);
        let simulate_number_of_days = 256;
        let mut new_fish = 0;
        for day in 0..simulate_number_of_days {
            new_fish = lantern_fish_occurrences[0];
            for fish_index in 1..lantern_fish_occurrences.len() {
                lantern_fish_occurrences[fish_index - 1] = lantern_fish_occurrences[fish_index];
            }
            lantern_fish_occurrences[6] += new_fish;
            lantern_fish_occurrences[8] = new_fish;
    
        }
    
        let mut sum: usize = 0;
        for fish_index in 0..lantern_fish_occurrences.len() {
            sum += lantern_fish_occurrences[fish_index];
        }
    
        println!("b_06_21:  Number of lantern fish: {} ", sum);
        sum
    }
}


fn a_06_21(use_functional: bool) -> usize {
    if use_functional {
        let lantern_fish = parse_txt_file_to_lantern_fish("C:/Programming/advent_of_code_rust/input/day6.txt");
        let buckets = 9;
        let mut lantern_fish_occurrences = int_vec_to_occurrences(buckets, &lantern_fish);
        let simulate_number_of_days = 80;

        for day in 0..simulate_number_of_days {
            lantern_fish_occurrences.rotate_left(1);
            lantern_fish_occurrences[6] += lantern_fish_occurrences[8];
        }
        
        lantern_fish_occurrences.iter().sum()

    } else {
        let mut lantern_fish : Vec<i32> = 
        parse_txt_file_to_lantern_fish("C:/Programming/advent_of_code_rust/input/day6.txt")
        .iter().map(|x| *x as i32).collect();

        let simulate_number_of_days = 80;
        for day in 0..simulate_number_of_days {
            let mut new_fish = Vec::<i32>::new();

            for fish_index in 0..lantern_fish.len() {
                lantern_fish[fish_index] -= 1;
                if lantern_fish[fish_index] == -1 {
                    new_fish.push(8);
                    lantern_fish[fish_index] = 6;
                }
            }

            lantern_fish.append(&mut new_fish);
        }


        println!("a_06_21:  Number of lantern fish: {} ", lantern_fish.len());

        lantern_fish.len()
    }
}


fn b_05_21(use_functional: bool) -> i32 {

    let steam_lines = parse_txt_file_to_steam_line_vec_usize("C:/Programming/advent_of_code_rust/input/day5.txt");
    
    let now = std::time::Instant::now();
    let max_value = steam_lines.iter().max().unwrap() + 1;
    let mut steam_map: SteamMap = initialize_steam_map(max_value as usize);
    let threshold = 2;

    // Do an iter version of this with chunk?
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

    println!("b_05_21: Number of overlaps {} ", number_of_overlaps);


    let elapsed_time = now.elapsed();
    println!("b_05_21: Running function b without file parsing took {} microseconds.", elapsed_time.as_micros());
    number_of_overlaps as i32
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

    println!("a_05_21: Number of overlaps {} ", number_of_overlaps);
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

    println!("b_04_21: Winning Score {}, Last winner: {}, Last draw {} ", winning_score, last_winner, last_draw);
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

    println!("a_04_21: Winning score was {}", winning_score);
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
fn test_b_10_21() {
    assert_eq!(b_10_21(true), 3490802734);
    assert_eq!(b_10_21(false), 3490802734);
}

#[test]
fn test_a_10_21() {
    assert_eq!(a_10_21(true), 294195);
    assert_eq!(a_10_21(false), 294195);
}

#[test]
fn test_b_09_21() {
    assert_eq!(b_09_21(true), 1047744);
    assert_eq!(b_09_21(false), 1047744);
}

#[test]
fn test_a_09_21() {
    assert_eq!(a_09_21(true), 456);
    assert_eq!(a_09_21(false), 456);
}

#[test]
fn test_b_08_21() {
    assert_eq!(b_08_21(true), 1051087);
    assert_eq!(b_08_21(false), 1051087);
}

#[test]
fn test_a_08_21() {
    assert_eq!(a_08_21(true), 530);
    assert_eq!(a_08_21(false), 530);
}

#[test]
fn test_b_07_21() {
    assert_eq!(b_07_21(true), 95851339);
    assert_eq!(b_07_21(false), 95851339);
}

#[test]
fn test_a_07_21() {
    assert_eq!(a_07_21(true), 335271);
    assert_eq!(a_07_21(false), 335271);
}

#[test]
fn test_b_06_21() {
    assert_eq!(b_06_21(true), 1640526601595);
    assert_eq!(b_06_21(false), 1640526601595);
}

#[test]
fn test_a_06_21() {
    assert_eq!(a_06_21(true), 362666);
    assert_eq!(a_06_21(false), 362666);
}

#[test]
fn test_b_05_21() {
    assert_eq!(b_05_21(true), 19349);
    assert_eq!(b_05_21(false), 19349);
}

#[test]
fn test_a_05_21() {
    assert_eq!(a_05_21(true), 6007);
    assert_eq!(a_05_21(false), 6007);
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

