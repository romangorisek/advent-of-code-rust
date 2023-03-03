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

fn sum_groups(lines: io::Lines<BufReader<File>>) -> Result<Vec<u32>, std::io::Error> {
    let mut sum: u32 = 0;
    let mut groups: Vec<u32> = Vec::new();
    for line in lines {
        let trimmed_line = line?.trim().to_string();
        if trimmed_line == "" {
            groups.push(sum);
            sum = 0;
        } else {
            sum += trimmed_line.parse::<u32>().unwrap();
        }
    }
    Ok(groups)
}

fn main() {
    let lines = read_lines("./input.txt".to_string());
    let sums = sum_groups(lines).unwrap();
    let max = sums.iter().max().unwrap();
    println!("{}", max);
}
