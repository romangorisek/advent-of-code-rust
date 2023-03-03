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

fn priority(c: char) -> usize {
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    match chars.find(c) {
        Some(i) => i + 1,
        None => 0,
    }
}

fn shared_item(line: &str) -> Option<char> {
    let (compartment1, compartment2) = line.split_at(line.len() / 2);
    for char1 in compartment1.chars() {
        for char2 in compartment2.chars() {
            if char1 == char2 {
                return Some(char1);
            }
        }
    }
    None
}

fn shared_item_for_group(lines: &[String; 3]) -> Option<char> {
    for char in lines[0].chars() {
        if lines[1].contains(char) && lines[2].contains(char) {
            return Some(char);
        }
    }
    None
}

fn main() {
    let lines = read_lines("./input.txt".to_string());
    let mut sum = 0;
    let mut count = 0;
    let mut batch = ["".to_string(), "".to_string(), "".to_string()];
    for line in lines {
        batch[count] = line.unwrap();
        count += 1;

        if count == 3 {
            let c = shared_item_for_group(&batch);
            let prio = priority(c.unwrap());
            sum += prio;
            count = 0;
        }
    }
    println!("{sum}");
}
