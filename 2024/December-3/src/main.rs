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
            // eprintln!("Error: No ',' character found in input: {}", input);
            return 0;
        }
    };

    // Get the substring on input from 0 to i and convert it to an integer
    let x = match input[0..i].parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            // eprintln!("Error parsing integer from input: {}: {}", &input[0..i], e);
            return 0;
        }
    };

    // find a closing parenthesis in the string
    let j = match input.find(')') {
        Some(index) => index,
        None => {
            // eprintln!("Error: No ')' character found in input: {}", input);
            return 0;
        }
    };

    // Get the substring on input from i+1 to j and convert it to an integer
    let y = match input[i+1..j].parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            // eprintln!("Error parsing integer from input: {}: {}", &input[i+1..j], e);
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





// Processes the input data
fn process_input_2(input: &str) -> i32 {

    let mut sop = 0;
    let mut input = input.to_string(); // Convert input to a mutable String
    let mut i = 0;

    // Repeat until no more "do()" or "don't()" are found
    loop {


        // Print the index of the first instance of 'do()'
//        println!("do() found at index: {}", i);

        // Find the first instance of the string 'don't()' that occurs after i in the input string
        let j = match input[i..].find("don't()") {
            Some(index) => index + i, // Adjust index to be relative to the original string
            None => {
                // eprintln!("Error: No 'don't()' character found in input: {}", input);
                return sop;
            }
        };

        // Print the index of the first instance of 'don't()'
//        println!("don't() found at index: {}", j);

        // Get the substring between i and j
        let s = &input[i..j];
println!("s: {}\n\n", s);
        // Process the substring
        sop += process_input(s);

//        println!("sop: {}", sop);

        // Find the first instance of the string 'do()' in the input string
        i = match input.find("do()") {
            Some(index) => index,
            None => {
                // eprintln!("Error: No 'do()' character found in input: {}", input);
                return sop;
            }
        };

        // Remove the processed part from the input string
        input = input[j..].to_string();





 /* 
        // Check if there are no more 'do()' or 'don't()' in the input
        if !input.contains("do()") || !input.contains("don't()") {
            return sop;
        }
    */
        
    }

/* 
    // Repeat forever
    loop {
        // Find the first instance of the string 'do()' in the input string
        if (i != 0) {
        //if true {
            i = match input.find("do()") {
                Some(index) => index,
                None => {
                    // eprintln!("Error: No do() character found in input: {}", input);
                    return sop;
                }
            };
        }

        // print the index of the first instance of 'do()'
//        println!("do() found at index: {}", i);

        // Find the first instance of the string 'don't()' that occurs after i in the input string
        let j = match input[i..].find("don't()") {
            Some(index) => index,
            None => {
                // eprintln!("Error: No don't() character found in input: {}", input);
                return sop;
            }
        };

        // Get the substring getween i and j
        let s = &input[i..i+j];

        // Process the substring
        sop += process_input(s);

        println!("sop: {}", sop);

        // Set input to the substring of input that occurs after j
        input = input[j..].to_string();
    }
    */
}



fn main() {

    // The input file is input.txt
    let infile = "input.txt";

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
