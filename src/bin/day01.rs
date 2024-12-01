use adventofcode2024::read_lines;

fn part1(filename: &str) {
    for line in read_lines(filename) {
        println!("{}", line);
    }
}

fn main() {
    part1("inputs/day01.txt");
}
