use std::time::Instant;
use adventofcode2024::input::AocInput;

fn is_safe(line: &Vec<i64>) -> bool {
    let mut min_step = std::i64::MAX;
    let mut max_step = std::i64::MIN;
    let mut it = line.iter();
    let mut last: &i64 = it.next().unwrap();
    for v in it {
        min_step = min_step.min(v - last);
        max_step = max_step.max(v - last);
        last = v;
    }
    let lower = min_step.abs().min(max_step.abs());
    let upper = min_step.abs().max(max_step.abs());
    (min_step * max_step > 0) & (lower > 0) & (upper < 4)
}

fn is_safe_dampened(line: &Vec<i64>) -> bool {
    for i in 0..line.len() {
        let mut l = line.clone();
        l.remove(i);
        if is_safe(&l) {
            return true;
        }
    }
    return false;
}

fn part1(i: &mut AocInput) {
    let cnt = i.read_vector_of_number_rows().into_iter().filter(is_safe).count();
    println!("Part 1: {}", cnt);
}

fn part2(i: &mut AocInput) {
    let cnt = i.read_vector_of_number_rows().into_iter().filter(is_safe_dampened).count();
    println!("Part 2: {}", cnt);
}

fn main() {
    let start = Instant::now();
    let mut i = AocInput::new("inputs/day02.txt");
    part1(&mut i);
    i.reset();
    part2(&mut i);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}