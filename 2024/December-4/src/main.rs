/*
    Advent of Code
    Day 4
    https://adventofcode.com/2024/day/4
*/

// Performs Part 1
fn process_part_1(input: &str) -> i32 {
    let mut xmas_count = 0;

    xmas_count
}








fn main() {
    // The input file is input.txt
//    let infile = "input.txt";
    let mut infile = "/Users/dbelt365/Code/aoc/2024/December-4/input.txt";
    
    // Read the input file into a string
    let input = std::fs::read_to_string(infile).unwrap();

    // Process the input data
    let value = process_part_1(&input);

    // Print the value
    println!("Part 1: {}", value);
/* 
    // Process the input data
    let value = process_input(&input);

    // Print the value
    println!("Part 1: {}", value);

    // Process the input data part 2
    let value = process_input_2(&input);

    // Print the value
    println!("Part 2: {}", value);
*/
}