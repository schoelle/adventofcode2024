use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn split_count(stone: i64, blinks: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(r) = cache.get(&(stone, blinks)) {
        return *r;
    }
    let stone_str = stone.to_string();
    let res = if blinks == 0 {
        1
    } else if stone == 0 {
        split_count(1, blinks - 1, cache)
    } else if stone_str.len() % 2 == 0 {
        let (left, right) = stone_str.split_at(stone_str.len() / 2);
        split_count(left.parse().unwrap(), blinks - 1, cache)
            + split_count(right.parse().unwrap(), blinks - 1, cache)
    } else {
        split_count(stone * 2024, blinks - 1, cache)
    };
    cache.insert((stone, blinks), res);
    res
}

fn part1(stones: &Vec<i64>) {
    let mut cache = HashMap::new();
    println!(
        "Part 1: {:?}",
        stones
            .iter()
            .map(|s| split_count(*s, 25, &mut cache))
            .sum::<i64>()
    );
}

fn part2(stones: &Vec<i64>) {
    let mut cache = HashMap::new();
    println!(
        "Part 1: {:?}",
        stones
            .iter()
            .map(|s| split_count(*s, 75, &mut cache))
            .sum::<i64>()
    );
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day11.txt");
    let mut stones = input.read_vector_of_number_rows()[0].clone();
    part1(&stones);
    part2(&stones);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
