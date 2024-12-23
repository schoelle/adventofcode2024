use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::collections::VecDeque;
use std::time::Instant;

fn diamond(n: i64) -> Vec<Pos> {
    let mut res = Vec::new();
    for x in -n..=n {
        for y in -n..=n {
            if x.abs() + y.abs() <= n {
                res.push(Pos(x, y));
            }
        }
    }
    res
}

fn distance_map(start: Pos, map: &Map) -> Vec<Vec<i64>> {
    let mut todo = VecDeque::new();
    let mut dist: Vec<Vec<i64>> = vec![vec![i64::MAX; map.width as usize]; map.height as usize];
    todo.push_back(start);
    dist[start.1 as usize][start.0 as usize] = 0;
    while let Some(p) = todo.pop_front() {
        for d in vec![Dir::N, Dir::E, Dir::W, Dir::S] {
            let t = p.step(d);
            if map.valid_pos(t)
                && map.get(t) == '.'
                && dist[p.1 as usize][p.0 as usize] < dist[t.1 as usize][t.0 as usize] - 1
            {
                dist[t.1 as usize][t.0 as usize] = dist[p.1 as usize][p.0 as usize] + 1;
                todo.push_back(t);
            }
        }
    }
    dist
}

fn picos(map: &Map, sdist: &Vec<Vec<i64>>, edist: &Vec<Vec<i64>>, max: i64, skip: i64) -> i64 {
    let offsets: Vec<Pos> = diamond(skip);

    let mut cnt = 0;
    for (p, _) in map.enumerate() {
        let sd = sdist[p.1 as usize][p.0 as usize];
        if sd < i64::MAX {
            for o in &offsets {
                let t = p + *o;
                if map.valid_pos(t) {
                    let td = edist[t.1 as usize][t.0 as usize];
                    if td < i64::MAX {
                        let dist = (t.0 - p.0).abs() + (t.1 - p.1).abs();
                        let total = sd + dist + td;
                        if total <= max - 100 {
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }
    cnt
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day20.txt");
    let mut map = input.read_map();
    let start_loc = *map.find('S').first().unwrap();
    let end_loc = *map.find('E').first().unwrap();
    map.set(start_loc, '.');
    map.set(end_loc, '.');

    let sdist = distance_map(start_loc, &map);
    let edist = distance_map(end_loc, &map);
    let max = sdist[end_loc.1 as usize][end_loc.0 as usize];

    println!("Part 1: {}", picos(&map, &sdist, &edist, max, 2));
    println!("Part 2: {}", picos(&map, &sdist, &edist, max, 20));

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
