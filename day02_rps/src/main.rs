use std::collections::HashMap;
use std::fs;

fn main() {
    // Read input
    let input_path = "input.txt";
    let mut data = fs::read_to_string(input_path).expect("Unable to read file");

    // trim data
    data = data.trim().to_string();

    // Split on double line break to get groups
    let split = data.split("\n");
    let hands: Vec<&str> = split.collect();

    strategy_one(&hands);
    strategy_two(&hands);
}

fn strategy_two(hands: &Vec<&str>) {
    let mut total_score: i32 = 0;
    for hand in hands {
        let values: Vec<&str> = hand.split(" ").collect();
        let villain_letter: &str = values[0];
        let hand_outcome: &str = values[1];
        let score: i32 = score_hand_two(villain_letter, hand_outcome);
        // println!(
        //     "Villain letter: {} \nHand Outcome: {} \nScore: {}",
        //     villain_letter, hand_outcome, score
        // );
        total_score += score
    }
    println!("Strategy 2 Total Score: {}", total_score)
}

fn score_hand_two(villain_letter: &str, outcome_letter: &str) -> i32 {
    let outcome: &str;
    let villain_shape: &str;
    let win_score: i32;
    let shape_score: i32;

    let letter_map = HashMap::from([
        ("A", "Rock"),
        ("B", "Paper"),
        ("C", "Scissors"),
        ("X", "Lose"),
        ("Y", "Draw"),
        ("Z", "Win"),
    ]);

    match letter_map.get(villain_letter) {
        Some(value) => villain_shape = value,
        None => villain_shape = "",
    }
    match letter_map.get(outcome_letter) {
        Some(value) => outcome = value,
        None => outcome = "",
    }

    // Scoring (my shape / win loss)
    // Rock = 1
    // Paper = 2
    // Scissors = 3

    match outcome {
        "Win" => {
            win_score = 6;
            match villain_shape {
                "Rock" => shape_score = 2,     // paper
                "Paper" => shape_score = 3,    // scissors
                "Scissors" => shape_score = 1, // rock
                _ => shape_score = 0,
            }
        }
        "Draw" => {
            win_score = 3;
            match villain_shape {
                "Rock" => shape_score = 1,     // rock
                "Paper" => shape_score = 2,    // paper
                "Scissors" => shape_score = 3, // scissors
                _ => shape_score = 0,
            }
        }
        "Lose" => {
            win_score = 0;
            match villain_shape {
                "Rock" => shape_score = 3,     // scissors
                "Paper" => shape_score = 1,    // rock
                "Scissors" => shape_score = 2, // paper
                _ => shape_score = 0,
            }
        }
        _ => {
            win_score = 0;
            shape_score = 0;
        }
    }

    shape_score + win_score
}

////////////////// Strategy One Below

fn strategy_one(hands: &Vec<&str>) {
    let mut total_score: i32 = 0;
    for hand in hands {
        let shapes: Vec<&str> = hand.split(" ").collect();
        let hero: &str = shapes[1];
        let villain: &str = shapes[0];
        let score: i32 = score_hand_one(hero, villain);
        total_score += score;
    }
    println!("Strategy 1 Total Score: {}", total_score);
}

fn score_hand_one(hero: &str, villain: &str) -> i32 {
    let hero_shape: &str;
    let villain_shape: &str;
    let win_score: i32;
    let shape_score: i32;

    let shape_map = HashMap::from([
        ("A", "R"),
        ("B", "P"),
        ("C", "S"),
        ("X", "R"),
        ("Y", "P"),
        ("Z", "S"),
    ]);

    match shape_map.get(hero) {
        Some(shape) => hero_shape = shape,
        None => hero_shape = "",
    }
    match shape_map.get(villain) {
        Some(shape) => villain_shape = shape,
        None => villain_shape = "",
    }

    match hero_shape {
        "R" => {
            shape_score = 1;
            match villain_shape {
                "R" => win_score = 3,
                "P" => win_score = 0,
                "S" => win_score = 6,
                _ => win_score = 0,
            }
        }
        "P" => {
            shape_score = 2;
            match villain_shape {
                "R" => win_score = 6,
                "P" => win_score = 3,
                "S" => win_score = 0,
                _ => win_score = 0,
            }
        }
        "S" => {
            shape_score = 3;
            match villain_shape {
                "R" => win_score = 0,
                "P" => win_score = 6,
                "S" => win_score = 3,
                _ => win_score = 0,
            }
        }
        _ => {
            shape_score = 0;
            win_score = 0;
        }
    }
    shape_score + win_score
}

// villain hand values
// A = Rock
// B = Paper
// C = Scissors

// hero hand values
// X = Rock
// Y = Paper
// Z = Scissors

// Scoring (my shape / win loss)
// Rock = 1
// Paper = 2
// Scissors = 3

// Win = 6
// Draw = 3
// Lose = 0
