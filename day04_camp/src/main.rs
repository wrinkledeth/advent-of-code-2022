use std::fs;

fn main() {
    // Read input and parse to string
    let input_path = "input.txt";
    let data = fs::read_to_string(input_path).expect("Unable to read file");

    let data = data.trim();
    let output: Vec<&str> = data.lines().collect();

    solve(&output);
}

fn solve(output: &Vec<&str>) {
    let mut fully_overlaps = 0;
    let mut overlaps = 0;

    for line in output {
        let elf_1 = line.split(",").collect::<Vec<&str>>()[0]
            .split("-")
            .collect::<Vec<&str>>();
        let elf_2 = line.split(",").collect::<Vec<&str>>()[1]
            .split("-")
            .collect::<Vec<&str>>();

        let elf_1_start = elf_1[0].parse::<i32>().unwrap();
        let elf_1_end = elf_1[1].parse::<i32>().unwrap();
        let elf_2_start = elf_2[0].parse::<i32>().unwrap();
        let elf_2_end = elf_2[1].parse::<i32>().unwrap();

        // fully overlaps
        if (elf_1_start <= elf_2_start && elf_1_end >= elf_2_end)
            || (elf_2_start <= elf_1_start && elf_2_end >= elf_1_end)
        {
            fully_overlaps += 1;
        }

        // overlaps
        if (elf_1_start <= elf_2_start && elf_1_end >= elf_2_start)
            || (elf_2_start <= elf_1_start && elf_2_end >= elf_1_start)
        {
            overlaps += 1;
        }
    }
    println!("Fully Overlaps: {}", fully_overlaps);
    println!("Overlaps: {}", overlaps);
}

// let cur = data[c];
// if marker.contains(&cur) {
//     counter = 0;
//     continue;
// } else {
//     counter += 1;
//     if counter == 4 {
//         println!("unique sequence: {:?}", &data[c - 3..c + 1]);
//         println!("solution: {:?}", c);
//         break;
//     }
// }
