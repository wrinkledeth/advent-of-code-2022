use std::collections::HashSet;
use std::fs;

fn main() {
    let input_path = "input.txt";
    let data = fs::read_to_string(input_path).expect("Unable to read file");

    let data = data.trim();

    // convert to vector of strings
    let rows: Vec<&str> = data.lines().collect();

    // split on " " and create a hashmap mapping the first element to second
    let mut move_list: Vec<(&str, i32)> = Vec::new();
    for row in rows {
        let row_vec: Vec<&str> = row.split(" ").collect();
        let direction = row_vec[0];
        let distance = row_vec[1].parse::<i32>().unwrap();
        move_list.push((direction, distance));
    }

    solve1(&move_list);
    solve2(&move_list);
}

fn solve2(move_list: &Vec<(&str, i32)>) {
    let mut head: (i32, i32) = (0, 0);
    // let mut tail: (i32, i32) = (0, 0);
    // create 10 tail nodes
    let mut tail_nodes: Vec<(i32, i32)> = Vec::new();
    for _ in 0..10 {
        tail_nodes.push((0, 0));
    }

    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    for moves in move_list.iter() {
        // println!("\n== {} {} ==", moves.0, moves.1);

        let mut movement_vec = (0, 0);
        match moves.0 {
            "R" => movement_vec = (1, 0),
            "U" => movement_vec = (0, 1),
            "L" => movement_vec = (-1, 0),
            "D" => movement_vec = (0, -1),
            _ => println!("Unknown direction"),
        }

        for _ in 0..moves.1 {
            for i in 0..tail_nodes.len() {
                // move head
                head = (head.0 + movement_vec.0, head.1 + movement_vec.1);

                let knight_jump: bool = (head.0 - tail_nodes[i].0).abs() == 2
                    && (head.1 - tail_nodes[i].1).abs() == 1
                    || (head.0 - tail_nodes[i].0).abs() == 1
                        && (head.1 - tail_nodes[i].1).abs() == 2;

                let cardinal_jump: bool = (head.0 - tail_nodes[i].0).abs() == 2
                    && (head.1 - tail_nodes[i].1).abs() == 0
                    || (head.0 - tail_nodes[i].0).abs() == 0
                        && (head.1 - tail_nodes[i].1).abs() == 2;

                if cardinal_jump {
                    tail_nodes[i] = (
                        tail_nodes[i].0 + movement_vec.0,
                        tail_nodes[i].1 + movement_vec.1,
                    );
                } else if knight_jump {
                    match moves.0 {
                        "R" => tail_nodes[i] = (tail_nodes[i].0 + 1, head.1),
                        "U" => tail_nodes[i] = (head.0, tail_nodes[i].1 + 1),
                        "L" => tail_nodes[i] = (tail_nodes[i].0 - 1, head.1),
                        "D" => tail_nodes[i] = (head.0, tail_nodes[i].1 - 1),
                        _ => println!("Unknown direction"),
                    }
                }
                // println!("head: {:?}, tail: {:?}", head, tail);
                seen.insert(tail_nodes[i]);
            }
        }
    }
    // println!("Seen\n{:?}", seen);
    println!("Answer2: {}", seen.len());
}

fn solve1(move_list: &Vec<(&str, i32)>) {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut seen: HashSet<(i32, i32)> = HashSet::new();

    // println!("Moves {:?}", move_list);

    for moves in move_list.iter() {
        // println!("\n== {} {} ==", moves.0, moves.1);

        let mut movement_vec = (0, 0);
        match moves.0 {
            "R" => movement_vec = (1, 0),
            "U" => movement_vec = (0, 1),
            "L" => movement_vec = (-1, 0),
            "D" => movement_vec = (0, -1),
            _ => println!("Unknown direction"),
        }

        for _ in 0..moves.1 {
            // move head
            head = (head.0 + movement_vec.0, head.1 + movement_vec.1);

            let knight_jump: bool = (head.0 - tail.0).abs() == 2 && (head.1 - tail.1).abs() == 1
                || (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 2;

            let cardinal_jump: bool = (head.0 - tail.0).abs() == 2 && (head.1 - tail.1).abs() == 0
                || (head.0 - tail.0).abs() == 0 && (head.1 - tail.1).abs() == 2;

            if cardinal_jump {
                tail = (tail.0 + movement_vec.0, tail.1 + movement_vec.1);
            } else if knight_jump {
                match moves.0 {
                    "R" => tail = (tail.0 + 1, head.1),
                    "U" => tail = (head.0, tail.1 + 1),
                    "L" => tail = (tail.0 - 1, head.1),
                    "D" => tail = (head.0, tail.1 - 1),
                    _ => println!("Unknown direction"),
                }
            }
            // println!("head: {:?}, tail: {:?}", head, tail);
            seen.insert(tail);
        }
    }
    // println!("Seen\n{:?}", seen);
    println!("Answer1: {}", seen.len());

    // print seen on a 2d grid, markned by #
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..10 {
        let mut row: Vec<char> = Vec::new();
        for j in 0..10 {
            if seen.contains(&(i, j)) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        grid.push(row);
    }

    // // print transposed grid
    // for i in (0..10).rev() {
    //     for j in 0..10 {
    //         print!("{}", &grid[j][i]);
    //     }
    //     println!("");
    // }
}
