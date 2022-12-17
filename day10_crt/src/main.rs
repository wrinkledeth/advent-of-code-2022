use std::fs;

fn main() {
    let input_path = "input.txt";
    let data = fs::read_to_string(input_path).expect("Unable to read file");

    let data = data.trim();

    // convert to vector of strings
    let rows: Vec<&str> = data.lines().collect();

    let cycle_x_value_list: Vec<(i32, i32)> = get_x_at_cycle(&rows);
    solve1(&cycle_x_value_list);
}

fn solve1(cycle_x_value_list: &Vec<(i32, i32)>) {
    let mut answer1: i32 = 0;
    let cycle_list = vec![20, 60, 100, 140, 180, 220];

    for (cycle, x) in cycle_x_value_list {
        println!("cycle: {} x: {}", cycle, x);
        let signal_strength: i32 = *cycle as i32 * *x as i32;
        if cycle_list.contains(cycle) {
            answer1 += signal_strength;
            println!(
                "Cycle: {} x: {} signal_strength {}",
                cycle, x, signal_strength
            );
        }
    }
    println!("Answer 1: {}", answer1);
}

fn get_x_at_cycle(rows: &Vec<&str>) -> Vec<(i32, i32)> {
    let mut x = 1;
    let mut cycle = 1;
    let instruction_len = rows.len();

    let mut addx_queue: Vec<(i32, i32)> = Vec::new();
    let mut cycle_x_value_list: Vec<(i32, i32)> = Vec::new();

    for i in 0..instruction_len + 4 {
        // if i < instruction_len {
        // println!("\nrow: {}", rows[i]);
        // }

        // add new signals to queue
        if i < instruction_len {
            let row_vec: Vec<&str> = rows[i].split(" ").collect();
            if row_vec[0] == "noop" {
                cycle += 1;
            } else if row_vec[0] == "addx" {
                cycle += 2;
                let signal = row_vec[1].parse::<i32>().unwrap();
                addx_queue.push((signal, 2))
            }
        }

        // println!("Cycle: {} x: {}", cycle, x);

        // decrement counters, add signals to x if counter is zero
        for j in 0..addx_queue.len() {
            addx_queue[j].1 -= 1;
            if addx_queue[j].1 == 0 {
                x += addx_queue[j].0;
                // println!("    x: {} addx: {}", x, addx_queue[j].0);
            }
        }
        // remove signals from queue if counter is zero
        addx_queue.retain(|&(_, counter)| counter != 0);

        // add cycle and x to list
        cycle_x_value_list.push((cycle, x));
    }
    cycle_x_value_list
}
