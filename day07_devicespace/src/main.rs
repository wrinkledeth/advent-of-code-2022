use std::collections::HashMap;
use std::fs;
use std::str;

fn main() {
    let input_path = "input.txt";
    let data = fs::read_to_string(input_path).expect("Unable to read file");

    let data = data.trim();
    let output: Vec<&str> = data.lines().collect();

    solve(&output);
}

fn solve(output: &Vec<&str>) {
    let mut current_dir_vec: Vec<&str> = Vec::new();
    let mut size_map: HashMap<String, i32> = HashMap::new();

    for line in output {
        let command: Vec<&str> = line.split(" ").collect();
        let first = command[0];
        let second = command[1];
        if first == "$" {
            if second == "cd" {
                let third = command[2];
                if third != { ".." } {
                    current_dir_vec.push(third);
                    let dir_path = current_dir_vec.join("/");
                    size_map.insert(dir_path, 0);
                } else {
                    current_dir_vec.pop();
                }
            }
        }
        if is_string_numeric(first.to_string()) {
            let dir_size = first.parse::<i32>().unwrap();
            let mut partial_dir_vec: Vec<&str> = Vec::new();
            for dir in &current_dir_vec {
                partial_dir_vec.push(dir);
                let dir_path = partial_dir_vec.join("/");
                let current_size = size_map.get(&dir_path).unwrap();
                size_map.insert(dir_path, current_size + dir_size);
            }
        }
    }

    // Part 1
    let mut answer = 0;
    for (dir, size) in size_map.iter() {
        if *size <= 100_000 {
            answer += *size;
        }
        // println!("{}: {}", dir, *size);
    }
    println!("Answer Part 1: {}", answer);

    // Part 2
    let install_size = 40_000_000;
    let space_used = size_map.get("/").unwrap();
    let to_delete = space_used - install_size;

    let candidates = size_map
        .iter()
        .filter(|(dir, size)| **size > to_delete)
        .collect::<Vec<(&String, &i32)>>();

    println!("Candidates: {:?}", candidates);

    // get candidate with minimum size
    let targe_folder = candidates
        .iter()
        .min_by_key(|(dir, size)| **size)
        .unwrap()
        .1;

    println!("Answer Part 2: {}", targe_folder);
}

fn is_string_numeric(input_string: String) -> bool {
    for c in input_string.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
