/*
    Advent of Code
    Day 1
    https://adventofcode.com/2024/day/1
*/

// Parse the input file into 2 list of integers
fn parse_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let input: Vec<&str> = input.split("\n").collect();

    let mut list_1: Vec<i32> = Vec::new();
    let mut list_2: Vec<i32> = Vec::new();
    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                (Ok(val1), Ok(val2)) => {
                    list_1.push(val1);
                    list_2.push(val2);
                }
                (Err(e1), _) => println!("Failed to parse '{}' as i32: {}", parts[0], e1),
                (_, Err(e2)) => println!("Failed to parse '{}' as i32: {}", parts[1], e2),
            }
        } else {
            println!("Invalid input line: '{}'", line);
        }
    }

    (list_1, list_2)
}

// Function to implement part 1 on the 2 lists
fn part_1(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {
    // Ensure both lists are sorted
    let mut list_1 = list_1.clone();
    let mut list_2 = list_2.clone();
    list_1.sort();
    list_2.sort();

    // Calculate the distance between each value in list_1 and list_2 and store in distance
    let mut distance: Vec<i32> = Vec::new();
    for i in 0..list_1.len() {
        distance.push((list_2[i] - list_1[i]).abs());
    }

    distance.iter().sum()
}

// Function to implement part 2 on the 2 lists
fn part_2(list_1: &Vec<i32>, list_2: &Vec<i32>) -> i32 {

    // For each value in list_1, count the number of occurences in list 2 then append to a new list called occurances
    let mut similarity: Vec<i32> = Vec::new();
    for i in 0..list_1.len() {
        let occurance = list_2.iter().filter(|&x| *x == list_1[i]).count() as i32;
        similarity.push(occurance*list_1[i]);
    }

    // Sum all of the values in the similarity list
    similarity.iter().sum()
}


/* Main entry point */
fn main() {

    // Set our input file to input.txt
    let input_file = "input.txt";

    // Parse the input file
    let (list_1, list_2) = parse_input(input_file);

    // Call part_1
    let result = part_1(&list_1, &list_2);

    // Call part_2
    let result_2 = part_2(&list_1, &list_2);

    println!(" Part 1 | Total Distance = {}", result);
    println!(" Part 2 | Similarity Score = {}", result_2);    
}
