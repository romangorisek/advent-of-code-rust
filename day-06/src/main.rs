use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: String) -> String {
    let mut file = File::open(filename).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data
}

fn start_of_packet(input: &String, size: usize) -> Option<usize> {
    let mut slice = HashMap::new();
    let byte_input = input.as_bytes();

    for i in size..input.len() {
        for j in 1..size + 1 {
            slice.insert(byte_input[i - j] as char, true);
        }
        println!("{:?}", slice);
        if slice.len() == size {
            return Some(i);
        } else {
            slice.clear();
        }
        println!("{:?}", slice);
    }

    None
}

fn main() {
    let input = read_file("./input.txt".to_string());
    let pos = start_of_packet(&input, 14).unwrap();
    println!("{pos}");
}
