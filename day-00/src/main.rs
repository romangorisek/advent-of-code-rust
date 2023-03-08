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


fn main() {
    let lines = read_lines("./input.txt".to_string());
    
    println!("{:?}", lines);
}
