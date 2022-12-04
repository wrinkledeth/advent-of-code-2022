use std::fs;

fn main() {
    // Read input and parse to string
    let input_path = "input.txt";
    let mut data = fs::read_to_string(input_path).expect("Unable to read file");

    // trim data
    data = data.trim().to_string();
    let split = data.split("\n");
    let output: Vec<&str> = split.collect();

    part_one(&output);
}

fn part_one(output: &Vec<&str>) {
    // println!("Part One: {:?}", output);
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
            // println!(
            //     "-----\nelf_1: {}, {} \nelf_2: {}, {}",
            //     elf_1_start, elf_1_end, elf_2_start, elf_2_end
            // );
            overlaps += 1;
        }
    }
    println!("Fully Overlaps: {}", fully_overlaps);
    println!("Overlaps: {}", overlaps);
}

// 719 too low
