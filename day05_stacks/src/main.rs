use std::fs;

fn main() {
    // Read input and parse to string
    let input_path = "input.txt";
    let data = fs::read_to_string(input_path).expect("Unable to read file");

    // let data = data.trim();
    let split = data.split("\n\n");
    let input_vec: Vec<&str> = split.collect();

    let crate_diagram: &str = input_vec[0];
    let crate_moves: &str = input_vec[1];

    let crate_diagram_vec = parse_crate_diagram(crate_diagram);
    let crate_moves_vec = parse_moves(crate_moves);

    solve1(&crate_diagram_vec, &crate_moves_vec);
    solve2(&crate_diagram_vec, &crate_moves_vec);
}

fn solve2(crate_diagram_vec: &Vec<Vec<char>>, crate_moves_vec: &Vec<Vec<usize>>) {
    let mut new_crate_vec = crate_diagram_vec.clone();

    for move_vec in crate_moves_vec {
        let amount = move_vec[0];
        let from = move_vec[1] - 1;
        let to = move_vec[2] - 1;

        // println!("From: {:?}, To: {:?}, Amount: {:?}", from, to, amount);
        // println!("From Stack (before): {:?}", new_crate_vec[from]);
        // println!("To Stack (before): {:?}", new_crate_vec[to]);

        // let continuous_stack = the last "amount" elements of the "from" stack
        let continuous_stack = new_crate_vec[from]
            .iter()
            .rev()
            .take(amount)
            .cloned()
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect::<Vec<char>>();

        // println!("Continuous Stack: {:?}", continuous_stack);

        for i in 0..amount {
            new_crate_vec[to].push(continuous_stack[i]);
            new_crate_vec[from].pop();
        }

        // println!("From Stack (after): {:?}", new_crate_vec[from]);
        // println!("To Stack (after): {:?}", new_crate_vec[to]);
        // break;
    }

    println!("Solution 2:");
    for row in new_crate_vec {
        // println!("{:?} ", row);
        if let Some(val) = row.last() {
            print!("{}", val);
        };
    }
    println!();
}

fn solve1(crate_diagram_vec: &Vec<Vec<char>>, crate_moves_vec: &Vec<Vec<usize>>) {
    let mut new_crate_vec = crate_diagram_vec.clone();

    for move_vec in crate_moves_vec {
        let amount = move_vec[0];
        let from = move_vec[1] - 1;
        let to = move_vec[2] - 1;

        for _ in 0..amount {
            let crate_letter = new_crate_vec[from].pop().unwrap();
            new_crate_vec[to].push(crate_letter);
        }
    }

    println!("Solution 1:");
    for row in new_crate_vec {
        // println!("{:?} ", row);
        if let Some(val) = row.last() {
            print!("{}", val);
        };
    }
    println!();
}

fn parse_moves(crate_moves: &str) -> Vec<Vec<usize>> {
    let mut crate_moves_vec = Vec::new();
    for line in crate_moves.lines() {
        let s = line
            .split(' ') // split on spaces
            .collect::<Vec<&str>>() // collect into vector
            .into_iter() // iterate over vector
            .filter(|s| s.parse::<usize>().is_ok()) // filter out non-numeric values
            .map(|s| s.parse::<usize>().unwrap()) // parse to usize
            .collect::<Vec<usize>>(); // collect into vector
        crate_moves_vec.push(s);
    }
    return crate_moves_vec;
}

fn parse_crate_diagram(crate_diagram: &str) -> Vec<Vec<char>> {
    println!("{}", "Parse Crate Diagram:");

    // created nested vector of chars
    let mut crate_vec: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        crate_vec.push(Vec::new());
    }

    let crate_diagram_rev = crate_diagram.lines().rev();
    for line in crate_diagram_rev {
        println!("{:?}", line);
    }

    for line in crate_diagram.lines().rev() {
        let chars: Vec<char> = line.chars().collect();
        if chars[1] == '1' {
            continue;
        }

        let valid_index = 1;
        for i in 0..9 {
            let crate_letter = chars[valid_index + i * 4];
            if crate_letter != ' ' {
                crate_vec[i].push(crate_letter);
            }
        }
    }

    for i in 0..9 {
        println!("{:?}", crate_vec[i]);
    }

    println!("-----------------------------------------");
    return crate_vec;
}
