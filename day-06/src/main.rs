use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename: String) -> String {
    let mut file = File::open(filename).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    data
}

fn start_of_packet(input: &String) -> Option<usize> {
    let mut slice = HashMap::new();
    let byte_input = input.as_bytes();

    for i in 13..input.len() {
        println!("{i}");
        println!("{}", &input[i - 10..i + 1]);
        slice.insert(byte_input[i] as char, true);
        slice.insert(byte_input[i - 1] as char, true);
        slice.insert(byte_input[i - 2] as char, true);
        slice.insert(byte_input[i - 3] as char, true);

        println!("{:?}", slice);
        if slice.len() == 4 {
            return Some(i + 1);
        } else {
            slice.clear();
        }
        println!("{:?}", slice);
    }

    None
}

fn main() {
    let input = read_file("./input.txt".to_string());
    let pos = start_of_packet(&input).unwrap();
    println!("{pos}");
}
