use crate::ascii::Map;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};

#[allow(dead_code)]
pub struct AocInput {
    filename: &'static str,
    content: Lines<BufReader<File>>,
    last_line: Option<String>,
}

#[allow(dead_code)]
impl AocInput {
    pub fn new(filename: &'static str) -> AocInput {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file).lines();
        AocInput {
            filename,
            content: reader,
            last_line: Some(String::from("")),
        }
    }

    pub fn reset(self: &mut AocInput) {
        let file = File::open(self.filename).unwrap();
        self.content = io::BufReader::new(file).lines();
        self.last_line = Some(String::from(""));
    }

    fn next_line(self: &mut AocInput) {
        let next = self.content.next();
        self.last_line = match next {
            Some(Ok(l)) => Some(l),
            Some(Err(_)) => None,
            None => None,
        }
    }

    fn has_data(self: &AocInput) -> bool {
        self.last_line.is_some() && !self.last_line.as_ref().unwrap().is_empty()
    }

    fn current(self: &AocInput) -> &String {
        self.last_line.as_ref().unwrap()
    }

    fn skip_empty(self: &mut AocInput) {
        while self.last_line.is_some() && self.last_line.as_ref().unwrap().is_empty() {
            self.next_line();
        }
    }

    pub fn read_line(self: &mut AocInput) -> String {
        self.skip_empty();
        let res = self.current().clone();
        self.next_line();
        res
    }

    pub fn read_lines(self: &mut AocInput) -> Vec<String> {
        self.skip_empty();
        let mut res = Vec::new();
        while self.has_data() {
            res.push(self.current().clone());
            self.next_line();
        }
        res
    }

    pub fn read_vector_of_numbers(self: &mut AocInput) -> Vec<i64> {
        self.skip_empty();
        let mut res: Vec<i64> = Vec::new();
        while self.has_data() {
            res.push(self.current().parse::<i64>().unwrap());
            self.next_line();
        }
        res
    }

    pub fn read_vector_of_number_pairs(self: &mut AocInput) -> (Vec<i64>, Vec<i64>) {
        self.skip_empty();
        let mut left: Vec<i64> = Vec::new();
        let mut right: Vec<i64> = Vec::new();
        while self.has_data() {
            let mut split = self.current().split_whitespace();
            left.push(split.next().unwrap().parse().unwrap());
            right.push(split.next().unwrap().parse().unwrap());
            self.next_line();
        }
        (left, right)
    }

    pub fn read_vector_of_number_rows(self: &mut AocInput) -> Vec<Vec<i64>> {
        self.skip_empty();
        let mut res = Vec::new();
        while self.has_data() {
            res.push(
                self.current()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            );
            self.next_line()
        }
        res
    }

    pub fn read_map(self: &mut AocInput) -> Map {
        self.skip_empty();
        let mut c: Vec<Vec<char>> = Vec::new();
        while self.has_data() {
            let line = self.read_line();
            c.push(line.chars().collect());
        }
        Map::new(c)
    }
}