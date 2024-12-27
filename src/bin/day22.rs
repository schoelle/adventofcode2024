use adventofcode2024::input::AocInput;
use std::collections::HashMap;
use std::time::Instant;

fn rng(i: i64) -> i64 {
    let a = ((i << 6) ^ i) & 0xFFFFFF;
    let b = ((a >> 5) ^ a) & 0xFFFFFF;
    let c = ((b << 11) ^ b) & 0xFFFFFF;
    c
}

fn prefix_dict(seq: &Vec<i64>) -> HashMap<[i64; 4], i64> {
    let mut res = HashMap::new();
    let mut g: [i64; 4] = [0, seq[1] - seq[0], seq[2] - seq[1], seq[3] - seq[2]];
    for i in 4..seq.len() {
        g[0] = g[1];
        g[1] = g[2];
        g[2] = g[3];
        g[3] = seq[i] - seq[i - 1];
        if !res.contains_key(&g) {
            res.insert(g, seq[i]);
        }
    }
    res
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day22.txt");

    let mut sum = 0;
    let mut seq: HashMap<i64, Vec<i64>> = HashMap::new();

    for line in input.read_lines() {
        let init = line.parse().unwrap();
        let mut price: i64 = init;
        let mut s = Vec::from([price.rem_euclid(10)]);
        for _ in 0..2000 {
            price = rng(price);
            s.push(price.rem_euclid(10));
        }
        seq.insert(init, s);
        sum += price;
    }
    println!("Part 1: {}", sum);

    let mut premap: HashMap<[i64; 4], Vec<i64>> = HashMap::new();
    for (_, v) in seq.iter() {
        let prefixes = prefix_dict(v);
        for (g, n) in prefixes {
            let vv = premap.entry(g).or_insert(Vec::new());
            vv.push(n);
        }
    }

    let mut best_total = 0;
    for (_, v) in &premap {
        let total: i64 = v.iter().sum();
        if total > best_total {
            best_total = total;
        }
    }
    println!("Part 2: {best_total}");

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
