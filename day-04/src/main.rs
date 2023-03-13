use regex::Regex;
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

fn fully_contains(line: &str) -> bool {
    let seperator = Regex::new(r"([-,])").expect("Invalid regex");
    let splits: Vec<usize> = seperator
        .split(line)
        .into_iter()
        .map(|i| i.parse::<usize>().unwrap())
        .collect();

    !(splits[2] > splits[1]) && !(splits[3] < splits[0])
}

fn main() {
    let lines = read_lines("./input.txt".to_string());
    let mut sum = 0;

    for line in lines {
        let i = line.as_ref().unwrap();
        if fully_contains(i) {
            // println!("{} ----", i);
            sum += 1;
        }
    }
    println!("{}", sum);
}
