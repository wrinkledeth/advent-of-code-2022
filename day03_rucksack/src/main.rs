use std::collections::HashMap;
use std::fs;

fn main() {
    // Read input and parse to string
    let input_path = "input.txt";
    let mut data = fs::read_to_string(input_path).expect("Unable to read file");

    // trim data
    data = data.trim().to_string();
    let split = data.split("\n");
    let output: Vec<&str> = split.collect();

    // create alphabet to index hashmap
    let mut alphabet: HashMap<char, usize> = HashMap::new();
    let mut index = 1;
    for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        alphabet.insert(c, index);
        index += 1;
    }

    part_one(&output, &alphabet);
    part_two(&output, &alphabet);
    // parse_input();
}

//! Wtf is going on here?
// fn parse_input() -> Vec<&str> {
//     // Read input and parse to string
//     let input_path = "input.txt";
//     let mut data = fs::read_to_string(input_path).expect("Unable to read file");

//     // trim data
//     data = data.trim().to_string();
//     let split = data.split("\n");
//     let output: Vec<&str> = split.collect();

//     output
// }

fn part_two(output: &Vec<&str>, alphabet: &HashMap<char, usize>) {
    let mut triplets: Vec<Vec<&str>> = Vec::new();
    let mut triplet: Vec<&str> = Vec::new();
    for line in output {
        triplet.push(line);
        if triplet.len() == 3 {
            triplets.push(triplet);
            triplet = Vec::new();
        }
    }

    let mut sum: usize = 0;
    for triplet in &triplets {
        // println!("Triplet: {:?}", triplet);
        let first = triplet[0];
        let second = triplet[1];
        let third = triplet[2];

        'outer: for i in first.chars() {
            for j in second.chars() {
                for k in third.chars() {
                    if i == j && j == k {
                        let priority = alphabet.get(&i).unwrap();
                        sum += priority;
                        // println!("Char: {}, Priority: {}", i, priority);
                        break 'outer;
                    }
                }
            }
        }
        // break;
    }

    // // print output length
    // println!("Output length: {}", &output.len());
    // // print triplets length
    // println!("Triplets length: {}", &triplets.len());
    // // print output / triplets
    // println!("Output / triplets: {}", &output.len() / &triplets.len());

    println!("Part two: {}", sum);
}

fn part_one(output: &Vec<&str>, alphabet: &HashMap<char, usize>) {
    let mut sum: usize = 0;
    for line in output {
        let priority = compute_priority(line, &alphabet);
        sum += priority;
    }

    println!("Part One: {}", sum);
}

fn compute_priority(line: &str, alphabet: &HashMap<char, usize>) -> usize {
    let length = line.len();

    let slice1 = &line[0..length / 2];
    let slice2 = &line[length / 2..length];

    let mut priority: usize = 0;

    for c1 in slice1.chars() {
        for c2 in slice2.chars() {
            if c1 == c2 {
                priority = *alphabet.get(&c1).unwrap();
                break;
            }
        }
    }
    return priority;
}
