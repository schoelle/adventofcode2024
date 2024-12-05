use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
use adventofcode2024::input::AocInput;

fn main() {
    let mut input = AocInput::new("inputs/day05.txt");
    let mut order: HashSet<(i64, i64)> = HashSet::new();
    input.read_vector_of_number_rows_by('|').iter().for_each(|v| {
        order.insert((v[0], v[1]));
    });
    let books = input.read_vector_of_number_rows_by(',');
    let mut part1 = 0;
    let mut part2 = 0;
    for book in books {
        let mut correct = true;
        for i in 0..book.len() {
            for j in i+1..book.len() {
                correct = correct && !order.contains(&(book[j].clone(), book[i].clone()))
            }
        }
        if correct {
            let middle = book[book.len() / 2];
            part1 += middle;
        } else {
            let mut srt = book.clone();
            srt.sort_by(|a,b|
                if order.contains(&(*a,*b)) {
                    Less
                } else if order.contains(&(*b,*a)) {
                    Greater
                } else {
                    Equal
                });
            let middle = srt[srt.len() / 2];
            part2 += middle;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
