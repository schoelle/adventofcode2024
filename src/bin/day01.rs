use adventofcode2024::read_lines;

fn read_input() -> (Vec<i64>, Vec<i64>) {
    let mut first_values = Vec::new();
    let mut second_values = Vec::new();
    for line in read_lines("inputs/day01.txt") {
        let mut split = line.split_whitespace();
        let first: i64 = split.next().unwrap().parse().unwrap();
        let second: i64 = split.next().unwrap().parse().unwrap();
        first_values.push(first);
        second_values.push(second);
    }
    (first_values, second_values)
}

fn part1() {
    let (mut first_values, mut second_values) = read_input();
    first_values.sort();
    second_values.sort();
    let both = first_values.iter().zip(second_values.iter());
    let res = both
        .map(|(first, second)| (first - second).abs())
        .sum::<i64>();
    println!("Part1: {}", res);
}

fn part2() {
    let (first_values, second_values) = read_input();
    let mut res: i64 = 0;
    for i in first_values {
        let occ = second_values.iter().filter(|j| i == **j).count() as i64;
        res += occ * i;
    }
    println!("Part2: {}", res);
}

fn main() {
    part1();
    part2();
}
