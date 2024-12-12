use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::collections::HashSet;
use std::time::Instant;

fn fill(start: Pos, map: &Map) -> HashSet<Pos> {
    let mut todo = Vec::new();
    let mut plot = HashSet::new();
    todo.push(start);
    while let Some(p) = todo.pop() {
        if plot.contains(&p) {
            continue;
        }
        plot.insert(p);
        for d in Vec::from([Dir::N, Dir::E, Dir::W, Dir::S]) {
            let t = p.step(d);
            if map.get(p) == map.get(t) {
                todo.push(t)
            }
        }
    }
    plot
}

fn calc_costs1(plot: &HashSet<Pos>, map: &Map) -> i64 {
    let area = plot.len() as i64;
    let fence: i64 = plot
        .iter()
        .map(|c| {
            Vec::from([Dir::N, Dir::E, Dir::W, Dir::S])
                .into_iter()
                .filter(|d| map.get(*c) != map.get(c.step(*d)))
                .count() as i64
        })
        .sum();
    area * fence
}

fn calc_costs2(plot: &HashSet<Pos>, map: &Map) -> i64 {
    let area = plot.len() as i64;
    let mut nfence: HashSet<Pos> = HashSet::new();
    let mut wfence: HashSet<Pos> = HashSet::new();
    let mut efence: HashSet<Pos> = HashSet::new();
    let mut sfence: HashSet<Pos> = HashSet::new();
    let mut ncount: i64 = 0;
    let mut scount: i64 = 0;
    let mut ecount: i64 = 0;
    let mut wcount: i64 = 0;
    for p in map
        .enumerate()
        .map(|(p, _)| p)
        .filter(|p| plot.contains(p))
    {
        if map.get(p.step(Dir::N)) != map.get(p) {
            nfence.insert(p);
            if !nfence.contains(&p.step(Dir::W)) {
                ncount += 1;
            }
        }
        if map.get(p.step(Dir::S)) != map.get(p) {
            sfence.insert(p);
            if !sfence.contains(&p.step(Dir::W)) {
                scount += 1;
            }
        }
        if map.get(p.step(Dir::E)) != map.get(p) {
            efence.insert(p);
            if !efence.contains(&p.step(Dir::N)) {
                ecount += 1;
            }
        }
        if map.get(p.step(Dir::W)) != map.get(p) {
            wfence.insert(p);
            if !wfence.contains(&p.step(Dir::N)) {
                wcount += 1;
            }
        }
    }
    area * (ncount + ecount + wcount + scount)
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day12.txt");
    let map = input.read_map();
    let mut done: HashSet<Pos> = HashSet::new();
    let mut costs1 = 0;
    let mut costs2 = 0;
    for (p, _) in map.enumerate() {
        if !done.contains(&p) {
            let plot = fill(p, &map);
            costs1 += calc_costs1(&plot, &map);
            costs2 += calc_costs2(&plot, &map);
            done.extend(&plot);
        }
    }
    println!("Part 1: {:?}", costs1);
    println!("Part 2: {:?}", costs2);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
