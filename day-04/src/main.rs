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

fn fully_contains(line: String) -> bool {
    let elves: Vec<&str> = line.split(",").collect();
    let mut elf1 = elves[0].split("-");
    let mut elf2 = elves[1].split("-");
    let elf1_min = elf1.nth(0).unwrap();
    let elf1_max = elf1.nth(2).unwrap();
    let elf2_min = elf2.nth(0).unwrap();
    let elf2_max = elf2.nth(2).unwrap();

    println!("{:?}", line);
    println!("{}-{},{}-{}", elf1_min, elf1_max, elf2_min, elf2_max);

    (elf1_min <= elf2_min && elf1_max >= elf2_max) || (elf2_min <= elf1_min && elf2_max >= elf1_max)
}

fn main() {
    let lines = read_lines("./input.txt".to_string());
    let mut sum = 0;
    for line in lines {
        if fully_contains(line.unwrap()) {
            sum += 0;
        }
    }
    println!("{}", sum);
}
