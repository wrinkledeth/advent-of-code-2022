use std::collections::HashMap;
use std::fs;
// use std::str;
use std::collections::HashSet;

fn main() {
    let input_path = "input.txt";
    let data = fs::read_to_string(input_path).expect("Unable to read file");

    let data = data.trim();

    // convert to vector of strings
    let string_rows: Vec<&str> = data.lines().collect();

    // convert each row into a vector of integers
    let mut rows = Vec::new();

    // collect rows
    // println!("Rows");
    for row in string_rows {
        let row_vec: Vec<i32> = row
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        // println!("{:?}", &row_vec);
        rows.push(row_vec);
    }

    // transpose rows to cols
    // println!("\nCols");
    let mut cols = Vec::new();
    for i in 0..rows[0].len() {
        let mut col = Vec::new();
        for row in &rows {
            col.push(row[i]);
        }
        // println!("{:?}", &col);
        cols.push(col);
    }

    solve1(&rows, &cols);
    solve2(&rows, &cols);
}

// find maximum scenic score
fn solve2(rows: &Vec<Vec<i32>>, cols: &Vec<Vec<i32>>) {
    let row_len = rows.len();
    let col_len = cols.len();

    // print rows
    // println!("\nRows");
    // for y in 0..row_len {
    // for x in 0..col_len {
    // print!("{} ", rows[y][x]);
    // }
    // println!();
    // }
    // maps coordinate to scenic score
    let mut score_map: HashMap<Vec<i32>, i32> = HashMap::new();

    for y in 0..row_len {
        for x in 0..col_len {
            let mut l_score = 0;
            let mut r_score = 0;
            let mut u_score = 0;
            let mut d_score = 0;
            let height = rows[y][x];
            // * check left
            for i in (0..x).rev() {
                if height > rows[y][i] {
                    l_score += 1;
                } else {
                    if i == 0 {
                        break;
                    }
                    l_score += 1;
                    break;
                }
            }
            // * check right
            for i in (x + 1)..col_len {
                if height > rows[y][i] {
                    r_score += 1;
                } else {
                    if i == col_len {
                        break;
                    }
                    r_score += 1;
                    break;
                }
            }
            // * check up
            for i in (0..y).rev() {
                if height > rows[i][x] {
                    u_score += 1;
                } else {
                    if i == 0 {
                        break;
                    }
                    u_score += 1;
                    break;
                }
            }
            // * check down
            for i in y + 1..row_len {
                if height > rows[i][x] {
                    d_score += 1;
                } else {
                    if i == row_len {
                        break;
                    }
                    d_score += 1;
                    break;
                }
            }

            // if y == 3 && x == 2 {
            //     println!(
            //         "y: {} \nx: {} \nL: {} \nR: {} \nU: {} \nD: {}",
            //         y, x, l_score, r_score, u_score, d_score
            //     );
            // }
            let total_score = l_score * r_score * u_score * d_score;
            score_map.insert(vec![y as i32, x as i32], total_score);
        }
    }

    // print score map line by line
    // println!("\nScore Map");
    // for y in 0..row_len {
    //     for x in 0..col_len {
    //         print!("{} ", score_map.get(&vec![y as i32, x as i32]).unwrap());
    //     }
    //     println!();
    // }
    let max_score = score_map.values().max().unwrap();
    println!("Answer 2: {}", max_score); //191 is too low
}

// trees visible from outride the grid
fn solve1(rows: &Vec<Vec<i32>>, cols: &Vec<Vec<i32>>) {
    // visible coordinates
    let mut visible_coordinates: HashSet<Vec<i32>> = HashSet::new();
    let mut visible_edges: Vec<Vec<i32>> = Vec::new();

    let row_len = rows.len();
    let col_len = cols.len();

    // println!("\nEdge Coords");
    // * populate visible coordinates with edges
    for (y, row) in rows.iter().enumerate() {
        for (x, height) in row.iter().enumerate() {
            if x == 0 || y == 0 || x == rows.len() - 1 || y == row.len() - 1 {
                let visible_coordinate = vec![y as i32, x as i32, *height];
                visible_coordinates.insert(visible_coordinate.clone());
                visible_edges.push(visible_coordinate.clone());
                // println!("{:?}", visible_coordinate)
            }
        }
    }

    for (y, row) in rows.iter().enumerate() {
        // * visible from the left
        for (x, height) in row.iter().enumerate() {
            // if height is greater than all elements to the left
            let mut is_visible = true;
            for i in 0..x {
                if height <= &row[i] {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                visible_coordinates.insert(vec![y as i32, x as i32, *height]);
                // println!("L: {:?}", vec![y as i32, x as i32, *height]);
            }
        }

        // * visible from the right
        for (x, height) in row.iter().rev().enumerate() {
            // if height is greater than all elements to the right
            // println!("y: {} x: {} height: {}", y, x, height);
            let mut is_visible = true;
            for i in 0..x {
                if height <= &row[row_len - i - 1] {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                let x_coord = row_len - x - 1;
                visible_coordinates.insert(vec![y as i32, x_coord as i32, *height]);
                // println!("R: {:?}", vec![y as i32, x_coord as i32, *height]);
            }
        }
    }

    for (x, col) in cols.iter().enumerate() {
        // * visible from the top
        for (y, height) in col.iter().enumerate() {
            // if height is greater than all elements above
            let mut is_visible = true;
            for i in 0..y {
                if height <= &col[i] {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                visible_coordinates.insert(vec![y as i32, x as i32, *height]);
                // println!("T: {:?}", vec![y as i32, x as i32, *height]);
            }
        }

        // * visible from the bottom
        for (y, height) in col.iter().rev().enumerate() {
            // if height is greater than all elements below
            let mut is_visible = true;
            for i in 0..y {
                if height <= &col[col_len - i - 1] {
                    is_visible = false;
                    break;
                }
            }
            if is_visible {
                let y_coord = col_len - y - 1;
                visible_coordinates.insert(vec![y_coord as i32, x as i32, *height]);
                // println!("B: {:?}", vec![y_coord as i32, x as i32, *height]);
            }
        }
    }

    // print visible coordinates not on edges
    // println!("\nVisible Coordinates");
    for visible_coordinate in &visible_coordinates {
        if !visible_edges.contains(visible_coordinate) {
            // println!("{:?}", visible_coordinate);
        }
    }

    println!("Answer 1: {}", visible_coordinates.len());
}

// 718 is wrong

// right middle 3 is the issue... its visible from the right even though its not the tallest
