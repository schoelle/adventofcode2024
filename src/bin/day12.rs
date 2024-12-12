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
    println!("Plot: {} {} {:?}", map.get(start), plot.len(), plot);
    plot
}


fn calc_costs(plot: &HashSet<Pos>, map: &&Map) -> i64 {
    let area = plot.len() as i64;
    let fence: i64 = plot.iter().map(|c|
        Vec::from([Dir::N, Dir::E, Dir::W, Dir::S]).into_iter().filter(|d| map.get(*c) != map.get(c.step(*d))).count() as i64
    ).sum();
    area * fence
}
fn part1(map: &Map) {
    let mut done: HashSet<Pos> = HashSet::new();
    let mut costs = 0;
    for (p, c) in map.enumerate() {
        if !done.contains(&p) {
            let plot = fill(p, &map);
            costs += calc_costs(&plot, &map);
            done.extend(&plot);
        }
    }
    println!("Part 1: {:?}", costs);
}


fn part2(stones: &Map) {
    println!("Part 2: {:?}", 2);
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day12.txt");
    let map = input.read_map();
    part1(&map);
    part2(&map);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
