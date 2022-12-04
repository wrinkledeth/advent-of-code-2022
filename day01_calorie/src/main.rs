use std::fs;
// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;

fn main() {
    // Read input
    let input_path = "input.txt";
    let mut data = fs::read_to_string(input_path).expect("Unable to read file");

    // trim data
    data = data.trim().to_string();

    // Split on double line break to get groups
    let split = data.split("\n\n");
    let vec: Vec<&str> = split.collect();

    // let mut max: u32 = 0;

    let mut sums: Vec<u32> = Vec::new();

    // For each calorie grouping, compute sum
    for group in vec {
        // Split subgroups by line break
        let string_vec: Vec<&str> = group.split("\n").collect();
        // println!("{:?}", string_vec);

        // convert vector of strings to integers
        let numbers: Result<Vec<u32>, _> = string_vec.iter().map(|x| x.parse()).collect();
        // println!("{:?}", numbers);

        // sum the numbers
        let sum: u32 = numbers.unwrap().iter().sum();

        sums.push(sum);
        // // update max
        // if sum > max {
        //     max = sum;
        // }
    }

    // sort the sums
    sums.sort();

    // sum the top 3
    println!("{:?}", sums[sums.len() - 3..].iter());
    println!("{:?}", sums[sums.len() - 3..].iter().sum::<u32>());
}
