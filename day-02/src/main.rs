use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn pick_to_points(pick: &str) -> usize {
    match pick {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!(),
    }
}

// ROCK A X
// PAPER B Y
// SCISSORS C Z
fn round_to_points(their_pick: &str, my_pick: &str) -> usize {
    let concat = format!("{}{}", their_pick, my_pick);
    match &concat[0..2] {
        "AZ" | "BX" | "CY" => 0,
        "AX" | "BY" | "CZ" => 3,
        "AY" | "BZ" | "CX" => 6,
        _ => panic!(),
    }
}

// part 2
// X -> loss
// Y -> draw
// Z -> win
fn outcome_to_points(outcome: &str) -> usize {
    match outcome {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    }
}

fn result_to_pick_points(their_pick: &str, outcome: &str) -> usize {
    let concat = format!("{}{}", their_pick, outcome);
    match &concat[0..2] {
        "AY" | "BX" | "CZ" => 1, // rock
        "AZ" | "BY" | "CX" => 2, // paper
        "AX" | "BZ" | "CY" => 3, // scissors
        _ => panic!(),
    }
}

fn calculate_score(lines: io::Lines<BufReader<File>>) -> usize {
    let mut score = 0;
    for res in lines {
        let line = res.unwrap();
        let their_pick = &line[..1];
        let my_pick = &line[2..];
        // score += round_to_points(their_pick, my_pick);
        // score += pick_to_points(my_pick);
        score += outcome_to_points(my_pick);
        score += result_to_pick_points(their_pick, my_pick);
    }
    score
}

fn main() {
    let lines = read_lines("./input.txt".to_string());
    let res = calculate_score(lines);
    println!("{}", res);
}
