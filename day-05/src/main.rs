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

fn parse_moves(line: &str) -> Vec<usize> {
    let splits: Vec<usize> = line
        .split(" ")
        .filter_map(|i| i.parse::<usize>().ok())
        .collect();
    splits
}

fn move_crates(stacks: &mut [Vec<&str>; 9], amount: usize, from: usize, to: usize) {
    for _ in 0..amount {
        let tmp = stacks[from].pop().unwrap();
        stacks[to].push(tmp);
    }
}

fn move_crates_2(stacks: &mut [Vec<&str>; 9], amount: usize, from: usize, to: usize) {
    let len = stacks[from].len() - amount;
    let pack = stacks[from][len..].to_vec();
    stacks[from] = stacks[from][..len].to_vec();
    for tmp in pack {
        stacks[to].push(tmp);
    }
}

fn print(stacks: &mut [Vec<&str>; 9]) {
    for i in 0..9 {
        let tmp = stacks[i].pop().unwrap();
        print!("{}", tmp);
    }
    println!();
}

fn main() {
    let mut stacks = [
        vec!["S", "C", "V", "N"],
        vec!["Z", "M", "J", "H", "N", "S"],
        vec!["M", "C", "T", "G", "J", "N", "D"],
        vec!["T", "D", "F", "J", "W", "R", "M"],
        vec!["P", "H", "H"],
        vec!["C", "T", "Z", "H", "J"],
        vec!["D", "P", "R", "Q", "F", "S", "L", "Z"],
        vec!["C", "S", "L", "H", "D", "F", "P", "W"],
        vec!["D", "S", "M", "P", "F", "N", "G", "Z"],
    ];

    let lines = read_lines("./input.txt".to_string());
    for line in lines {
        let i = line.as_ref().unwrap();
        let moves = parse_moves(i);
        move_crates_2(&mut stacks, moves[0], moves[1] - 1, moves[2] - 1);
    }

    print(&mut stacks);
}
