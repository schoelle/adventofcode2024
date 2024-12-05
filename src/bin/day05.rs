use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashSet;
use adventofcode2024::input::AocInput;

fn sort_book(book: &mut Vec<i64>, order: &HashSet<(i64,i64)>) {
    book.sort_by(|a,b|
        if order.contains(&(*a,*b)) {
            Less
        } else if order.contains(&(*b,*a)) {
            Greater
        } else {
            Equal
        })
}

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
        for (pos1, value1) in book.iter().enumerate() {
            for (pos2, value2) in book.iter().enumerate() {
                let pair = (value2.clone(), value1.clone());
                if pos1 < pos2 && order.contains(&pair) {
                    correct = false;
                }
            }
        }
        if correct {
            let middle = book[book.len() / 2];
            part1 += middle;
        } else {
            let mut srt = book.clone();
            sort_book(&mut srt, &order);
            let middle = srt[srt.len() / 2];
            part2 += middle;
        }

    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
