use adventofcode2024::input::AocInput;
use std::time::Instant;

fn rng(i: i64) -> i64 {
    let a = ((i * 64) ^ i) & (2i64.pow(24)-1);
    let b = ((a / 32) ^ a) & (2i64.pow(24)-1);
    let c = ((b * 2048) ^ b) & (2i64.pow(24)-1);
    c
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day22.txt");

    let mut sum = 0;

    for line in input.read_lines() {
        let mut price: i64 = line.parse().unwrap();
        for _ in 0..2000 {
            price = rng(price);
        }
        println!("{:?} {:?}", line, price);
        sum += price;
    }

    println!("Part 1: {}", sum);
    
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
