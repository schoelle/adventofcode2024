use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let mut result = Vec::new();
    for line in io::BufReader::new(file).lines() {
        result.push(line.unwrap());
    }
    return result;
}

