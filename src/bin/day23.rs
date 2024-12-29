use adventofcode2024::input::AocInput;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

type Nodes = HashSet<String>;
type Edges = HashMap<String, Nodes>;

fn bron_kerbosch(graph: &Edges, r: Nodes, mut p: Nodes, mut x: Nodes, cliques: &mut Vec<Nodes>) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
    } else if !p.is_empty() {
        let nodes = p.iter().cloned().collect::<Nodes>();
        for node in nodes {
            let neighbours: &Nodes = graph.get(&node).unwrap();
            let mut to_add: Nodes = Nodes::new();
            to_add.insert(node.clone());
            bron_kerbosch(
                graph,
                r.union(&to_add).cloned().collect(),
                p.intersection(&neighbours).cloned().collect(),
                x.intersection(&neighbours).cloned().collect(),
                cliques,
            );
            p.remove(&node);
            x.insert(node.clone());
        }
    }
}

fn part1(graph: &Edges, nodes: &Nodes) {
    let mut cnt = 0i64;
    for a in nodes {
        for b in nodes {
            if b > a {
                for c in nodes {
                    if c > b
                        && graph.get(a).unwrap().contains(b)
                        && graph.get(b).unwrap().contains(c)
                        && graph.get(c).unwrap().contains(a)
                    {
                        if a.starts_with("t") || b.starts_with("t") || c.starts_with("t") {
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", cnt);
}

fn part2(graph: &Edges, nodes: &Nodes) {
    let mut cliques = Vec::new();
    bron_kerbosch(
        &graph,
        Nodes::new(),
        nodes.clone(),
        Nodes::new(),
        &mut cliques,
    );
    let mut largest: Vec<String> = Vec::new();
    for c in cliques {
        if c.len() > largest.len() {
            largest = c.into_iter().collect();
        }
    }
    largest.sort();
    println!("Part 2: {}", &largest.join(","));
}

fn main() {
    let start = Instant::now();
    let mut input = AocInput::new("inputs/day23.txt");

    let mut nodes: Nodes = Nodes::new();
    let mut graph: Edges = Edges::new();

    let re = Regex::new("(..)-(..)").unwrap();
    for line in input.read_lines() {
        if let Some(m) = re.captures(&line) {
            let first = m.get(1).unwrap().as_str().to_string();
            let second = m.get(2).unwrap().as_str().to_string();
            graph
                .entry(first.clone())
                .or_insert(Nodes::new())
                .insert(second.clone());
            graph
                .entry(second.clone())
                .or_insert(Nodes::new())
                .insert(first.clone());
            nodes.insert(first);
            nodes.insert(second);
        }
    }

    part1(&graph, &nodes);
    part2(&graph, &nodes);

    let duration = start.elapsed();
    println!("Time: {:?}", duration);
}
