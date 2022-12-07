use std::collections::HashSet;
use std::fs;

fn main() {
    // Read input and parse to string
    let input_path = "input.txt";
    let data = fs::read_to_string(input_path).expect("Unable to read file");

    let data = data.trim().chars().collect::<Vec<char>>();

    solve1(&data);
    solve2(&data);
}

fn solve2(data: &Vec<char>) {
    let mut start = 0;
    let mut end = 14;

    for _ in 0..data.len() {
        let mut seen = HashSet::new();
        for i in start..end {
            if seen.contains(&&data[i]) {
                // println!("bad marker: {:?}", &data[start..end]);
                continue;
            }
            seen.insert(&data[i]);
            if seen.len() == 14 {
                // println!("Good marker: {:?}", &data[start..end]);
                println!("Part 2: {}", end);
                return;
            }
        }
        start += 1;
        end += 1;
    }
}

// 4 distinct characters
fn solve1(data: &Vec<char>) {
    let mut start = 0;
    let mut end = 4;

    for _ in 0..data.len() {
        let mut seen = HashSet::new();
        for i in start..end {
            if seen.contains(&&data[i]) {
                // println!("bad marker: {:?}", &data[start..end]);
                continue;
            }
            seen.insert(&data[i]);
            if seen.len() == 4 {
                // println!("Good marker: {:?}", &data[start..end]);
                println!("Part 2: {}", end);
                return;
            }
        }
        start += 1;
        end += 1;
    }
}
