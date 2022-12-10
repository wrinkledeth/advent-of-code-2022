// use std::collections::HashMap;
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
    println!("Rows");
    for row in string_rows {
        let row_vec: Vec<i32> = row
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .collect();
        println!("{:?}", &row_vec);
        rows.push(row_vec);
    }

    // transpose rows to cols
    println!("\nCols");
    let mut cols = Vec::new();
    for i in 0..rows[0].len() {
        let mut col = Vec::new();
        for row in &rows {
            col.push(row[i]);
        }
        println!("{:?}", &col);
        cols.push(col);
    }

    solve(&rows, &cols)
}

fn solve(rows: &Vec<Vec<i32>>, cols: &Vec<Vec<i32>>) {
    // visible coordinates
    let mut visible_coordinates: HashSet<Vec<i32>> = HashSet::new();

    let mut visible_edges: Vec<Vec<i32>> = Vec::new();
    let mut visible_coordinates_left: Vec<Vec<i32>> = Vec::new();
    let mut visible_coordinates_right: Vec<Vec<i32>> = Vec::new();
    let mut visible_coordinates_top: Vec<Vec<i32>> = Vec::new();
    let mut visible_coordinates_bottom: Vec<Vec<i32>> = Vec::new();

    println!("\nEdge Coords");
    // * populate visible coordinates with edges
    for (x, row) in rows.iter().enumerate() {
        for (y, height) in row.iter().enumerate() {
            if x == 0 || y == 0 || x == rows.len() - 1 || y == row.len() - 1 {
                let visible_coordinate = vec![x as i32, y as i32, *height];
                visible_coordinates.insert(visible_coordinate.clone());
                visible_edges.push(visible_coordinate.clone());
                println!("{:?}", visible_coordinate)
            }
        }
    }
    println!("{:?}", visible_coordinates.len());

    for (y, row) in rows.iter().enumerate() {
        let tallest = row.iter().max().unwrap();

        // * visible from the left
        let mut visible_left: Vec<i32> = Vec::new();
        for (x, height) in row.iter().enumerate() {
            if height == tallest {
                visible_left = vec![y as i32, x as i32, *tallest];
                break;
            }
        }
        visible_coordinates.insert(visible_left.clone());
        visible_coordinates_left.push(visible_left.clone());

        // * visible from the right
        let mut visible_right: Vec<i32> = Vec::new();
        for (x, height) in row.iter().enumerate() {
            if height == tallest {
                visible_right = vec![y as i32, x as i32, *tallest];
            }
        }
        visible_coordinates.insert(visible_right.clone());
        visible_coordinates_right.push(visible_right.clone());
    }

    for (x, col) in cols.iter().enumerate() {
        let tallest = col.iter().max().unwrap();

        // * visible from the top
        let mut visible_top: Vec<i32> = Vec::new();
        for (y, height) in col.iter().enumerate() {
            if height == tallest {
                visible_top = vec![y as i32, x as i32, *tallest];
                break;
            }
        }
        visible_coordinates.insert(visible_top.clone());
        visible_coordinates_top.push(visible_top.clone());

        // *  visible from the bottom
        let mut visible_bottom: Vec<i32> = Vec::new();
        for (y, height) in col.iter().enumerate() {
            if height == tallest {
                visible_bottom = vec![y as i32, x as i32, *tallest];
            }
        }
        visible_coordinates.insert(visible_bottom.clone());
        visible_coordinates_bottom.push(visible_bottom.clone());
    }

    // println!("left {:?}", visible_coordinates_left);
    // println!("right {:?}", visible_coordinates_right);
    // println!("top {:?}", visible_coordinates_top);
    // println!("bottom {:?}", visible_coordinates_bottom);

    // println!("Visible Coordinates\n{:?}", visible_coordinates);
    // println!("{:?}", visible_coordinates.len());

    // print visible coordinates not on edges
    println!("\nVisible Coordinates");
    for visible_coordinate in &visible_coordinates {
        if !visible_edges.contains(visible_coordinate) {
            println!("{:?}", visible_coordinate);
        }
    }
}

// 718 is wrong

// right middle 3 is the issue... its visible from the right even though its not the tallest
