/*
    Advent of Code
    Day 4
    https://adventofcode.com/2024/day/4
*/


// Import the array crate
use ndarray::Array;

// Constants
const XMAS: &str = "XMAS";
const XMAS_LEN: i32 = XMAS.len() as i32;
const DIRECTION_VECTORS: [(i32, i32); 8] = [(0, 1), // North
                                            (0, -1), // South
                                            (1, 0), // East
                                            (-1, 0), // West
                                            (-1, -1), // South West
                                            (-1, 1), // South East
                                            (1, -1), // Morth West
                                            (1, 1)]; // North East

// Function to count the number of XMAS strings at a given coordinate
fn count_xmas(position: (i32, i32), matrix: &Array<char, ndarray::Dim<[usize; 2]>>) -> i32 {
    let mut count = 0;

    // Get the dimensions of the matrix
    let dimensions = matrix.dim();

    // Loop through the direction vectors
    for direction in DIRECTION_VECTORS.iter() {
        let mut xmas_string = String::new();
        for i in 0..XMAS_LEN {
            let x = position.0 + i * direction.0;
            let y = position.1 + i * direction.1;
            if x >= 0 && y >= 0 && x < dimensions.0 as i32 && y < dimensions.1 as i32 {
                xmas_string.push(matrix[[x as usize, y as usize]]);
            } else {
                break;
            }
        }

        if xmas_string == XMAS {
            count += 1;
        }
    }

    count
}

// Processes part 1 of the problem
fn process_part_1(input: &str) -> i32 {
    let mut xmas_count = 0;

    let lines: Vec<&str> = input.lines().collect();

    // Create a 2D array with ndarray
    let mut xmas_matrix = Array::from_elem((lines.len(), lines[0].len()), ' ');

    // Loop through the lines and load the characters into the matrix
    for i in 0..lines.len() {
        let line = lines[i];
        for j in 0..line.len() {
            xmas_matrix[[i, j]] = line.chars().nth(j).unwrap();
        }
    }

    // Walk through the matrix and check each value for 'X'
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if xmas_matrix[[i, j]] == 'X' {
                xmas_count += count_xmas((i.try_into().unwrap(), j.try_into().unwrap()), &xmas_matrix);
            }
        }
    }

    xmas_count
}


// Main entry point
fn main() {
    // The input file is input.txt
    let infile = "input.txt";
//    let infile = "/Users/dbelt365/Code/aoc/2024/December-4/input.txt";
    
    // Read the input file into a string
    let input = std::fs::read_to_string(infile).unwrap();

    // Process the input data
    let value = process_part_1(&input);

    // Print the value
    println!("Part 1: {}", value);

}