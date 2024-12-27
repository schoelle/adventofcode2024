use adventofcode2024::input::AocInput;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day23.txt");

    let mut graph: HashSet<(String, String)> = HashSet::new();
    let mut nodes: HashSet<String> = HashSet::new();

    let re = Regex::new("(..)-(..)").unwrap();
    for line in input.read_lines() {
        if let Some(m) = re.captures(&line) {
            let first = m.get(1).unwrap().as_str().to_string();
            let second = m.get(2).unwrap().as_str().to_string();
            graph.insert((first.clone(), second.clone()));
            graph.insert((second.clone(), first.clone()));
            nodes.insert(first);
            nodes.insert(second);
        }
    }

    let mut cnt = 0;
    for a in &nodes {
        for b in &nodes {
            if b > a {
                for c in &nodes {
                    if c > b
                        && graph.contains(&(a.clone(), b.clone()))
                        && graph.contains(&(b.clone(), c.clone()))
                        && graph.contains(&(c.clone(), a.clone()))
                    {
                        if a.starts_with("t") || b.starts_with("t") || c.starts_with("t") {
                            println!("{} {} {}", a, b, c);
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", cnt);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
