use adventofcode2024::read_lines;

fn is_safe(line: &Vec<i64>) -> bool {
    let mut min_step = std::i64::MAX;
    let mut max_step = std::i64::MIN;
    let mut it = line.iter();
    let mut last: &i64 = it.next().unwrap();
    for v in it {
        min_step = min_step.min(v - last);
        max_step = max_step.max(v - last);
        last = v;
    }
    let lower = min_step.abs().min(max_step.abs());
    let upper = min_step.abs().max(max_step.abs());
    (min_step * max_step > 0) & (lower > 0) & (upper < 4)
}

fn is_safe_dampened(line: &Vec<i64>) -> bool {
    for i in 0..line.len() {
        let mut l = line.clone();
        l.remove(i);
        if is_safe(&l) {
            return true;
        }
    }
    return false;
}

fn part1() {
    let cnt = read_lines("inputs/day02.txt").iter().map(
        |l| l.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
    ).filter(is_safe).count();
    println!("Part 1: {}", cnt);
}

fn part2() {
    let cnt = read_lines("inputs/day02.txt").iter().map(
        |l| l.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()
    ).filter(is_safe_dampened).count();
    println!("Part 2: {}", cnt);
}

fn main() {
    part1();
    part2();
}
