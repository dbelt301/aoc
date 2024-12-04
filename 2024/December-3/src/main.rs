/*
    Advent of Code
    Day 3
    https://adventofcode.com/2024/day/3
*/

fn process_single_mul(input: &str) -> i32 {

    // Print the input
//    println!("{}", input);

    // Find a comma in the string
    let i = match input.find(',') {
        Some(index) => index,
        None => {
            return 0;
        }
    };

    // Get the substring on input from 0 to i and convert it to an integer
    let x = match input[0..i].parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            return 0;
        }
    };

    // find a closing parenthesis in the string
    let j = match input.find(')') {
        Some(index) => index,
        None => {
            return 0;
        }
    };

    // Get the substring on input from i+1 to j and convert it to an integer
    let y = match input[i+1..j].parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            return 0;
        }
    };

    // Multiply x and y and return the product
    let product = x * y;

    product
}

// Processes the input data
fn process_input(input: &str) -> i32 {
    let mut sop = 0;

    // Find every instance of the sequence 'mul' in the string and append its position to the list 'mul_pos'
    for (i, _) in input.match_indices("mul(") {
        // Find the next occurance of the character ')' after the sequence 'mul' and return a substring between the two
        let s = match input[i..].find(')') {
            Some(index) => index,
            None => {
            // eprintln!("Error: No ')' character found after 'mul(' in input: {}", input);
            return sop;
            }
        };

        // Capture the sub-string between i and i+s
        let s = &input[i+4..i+s+1];

        sop += process_single_mul(s);
    }    

    sop
}

// Processes the input data for part 2
fn process_input_2(input: &str) -> i32 {
    let mut sop = 0;
    let mut do_pos = Vec::new();
    let mut dont_pos = Vec::new();

    // Find every instance of the sequence 'do(' in the input and append its position to the list 'do_pos'
    for (i, _) in input.match_indices("do(") {
        do_pos.push(i);
    }

    // Find every instance of the sequence 'don't(' in the input and append its position to the list 'dont_pos'
    for (i, _) in input.match_indices("don't(") {
        dont_pos.push(i);
    }
    
    // Create an integer variable called start and assign to 0
    let mut start = 0;

    // set end to the first value in the list 'dont_pos'
    let mut end = dont_pos[0];

    // loop while start is less than the length of the input
    while start < input.len() {
        // Get the substring between start and end
        let s = &input[start..end];

        // Process the substring
        sop += process_input(s);

        // Set start to minimum value in the list 'do_pos' that's greater than end
        start = *do_pos.iter().filter(|&x| *x > end).min().unwrap_or(&input.len());

        // Set end to minimum value in the list 'dont_pos' that's greater than start
        end = *dont_pos.iter().filter(|&x| *x > start).min().unwrap_or(&input.len());

    }

    sop
}


// Main entry point
fn main() {

    // The input file is input.txt
//    let infile = "input.txt";
    let mut infile = "/Users/dbelt365/Code/aoc/2024/December-3/input.txt";
    
    // Read the input file into a string
    let input = std::fs::read_to_string(infile).unwrap();

    // Process the input data
    let value = process_input(&input);

    // Print the value
    println!("Part 1: {}", value);

    // Process the input data part 2
    let value = process_input_2(&input);

    // Print the value
    println!("Part 2: {}", value);

}
