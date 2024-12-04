/*
    Advent of Code
    Day 2
    https://adventofcode.com/2024/day/2
*/

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Parses the input file into a vector of strings
fn parse_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}


// Function that accepts a list of integers and returns a boolean if safe
fn is_safe_list_1(numbers: Vec<i32>) -> bool {
    // Define the criteria for a safe list
    // For example, a list is safe if all numbers are positive
    let mut safe = true;

    // Check for appropriate differences between numbers
    for i in 0..numbers.len() - 1 {
        let diff = (numbers[i] - numbers[i + 1]).abs();
        if diff < 1 || diff > 3 {
            safe = false;
        }
    }

    // Check if the list is in ascending or descending order
    let ascending = numbers.windows(2).all(|w| w[0] <= w[1]);
    let descending = numbers.windows(2).all(|w| w[0] >= w[1]);

    if !ascending && !descending {
        safe = false;
    }

    safe
}

// Function that accepts a list of integers and returns a boolean if safe
fn is_safe_list_2(numbers: Vec<i32>) -> bool {
    let mut safe = false;

    // For each element in the list
    for i in 0..numbers.len() {
        // Create a sub list with element i removed
        let mut sub_list = numbers.clone();
        sub_list.remove(i);

        safe = is_safe_list_1(sub_list);
        if safe {
            break;
        }
    }

    safe
}

// Function that accepts a list of strings and a function pointer to is_safe_list, then returns an integer
fn test_lists(strings: &Vec<String>, is_safe_list_fn: fn(Vec<i32>) -> bool) -> usize {
    let mut good_lists = 0;
    for string in strings {
        let mut numbers: Vec<i32> = Vec::new();
        let parts: Vec<&str> = string.split_whitespace().collect();
        for part in parts {
            match part.parse::<i32>() {
                Ok(val) => numbers.push(val),
                Err(e) => println!("Failed to parse '{}' as i32: {}", part, e),
            }
        }

        // Call the function pointer to determine if the list is safe
        if is_safe_list_fn(numbers) {
            good_lists += 1;
        }
    }

    good_lists
}



// Main entry point
fn main() {
    // Set the input file
    let filename = "input.txt";

    // Call parse_file to read the file and store the lines in a vector
    let lines = parse_file(filename).unwrap();

    // Call test_list to test the vector of strings and print the result
    let mut result = test_lists(&lines, is_safe_list_1);
    println!(" Part 1 | Number of good lists: {}", result);

    // Call test_list to test the vector of strings and print the result
    result = test_lists(&lines, is_safe_list_2);
    println!(" Part 2 | Number of good lists: {}", result);
}
