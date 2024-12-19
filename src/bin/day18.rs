use std::collections::VecDeque;
use adventofcode2024::ascii::{Dir, Map, Pos};
use adventofcode2024::input::AocInput;
use std::time::Instant;

const MAP_SIZE: i64 = 71;

fn path_length(map: &Map) -> i64 {
    let mut todo = VecDeque::new();
    let mut dist: Vec<Vec<i64>> = vec![vec![i64::MAX; MAP_SIZE as usize]; MAP_SIZE as usize];
    todo.push_back(Pos(0, 0));
    dist[0][0] = 0;
    while let Some(p) = todo.pop_front() {
        for d in vec![Dir::N, Dir::E, Dir::W, Dir::S] {
            let t = p.step(d);
            if map.valid_pos(t)
                && map.get(t) == '.'
                && dist[p.0 as usize][p.1 as usize] < dist[t.0 as usize][t.1 as usize] - 1
            {
                dist[t.0 as usize][t.1 as usize] = dist[p.0 as usize][p.1 as usize] + 1;
                todo.push_back(t);
            }
        }
    }
    dist[MAP_SIZE as usize - 1][MAP_SIZE as usize - 1]
}

fn part1(bytes: &Vec<Pos>) {
    let mut map = Map::new(MAP_SIZE, MAP_SIZE, '.');
    bytes.iter().take(1024).for_each(|p| map.set(*p, '#'));
    println!("Part 1: {}", path_length(&map));
}
fn part2(bytes: &Vec<Pos>) {
    let mut map = Map::new(MAP_SIZE, MAP_SIZE, '.');
    
    for b in bytes {
        map.set(*b, '#');
        if path_length(&map) == i64::MAX {
            println!("Part 2: {},{}", b.0, b.1);
            return;
        }
    }
}
fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day18.txt");
    let bytes: Vec<Pos> = input
        .read_vector_of_number_rows_by(',')
        .iter()
        .map(|r| Pos(r[0], r[1]))
        .collect();

    part1(&bytes);
    part2(&bytes);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
